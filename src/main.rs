use clap::Parser;
use hound::{Error, WavReader, WavSpec, WavWriter};
use rubato::{FftFixedInOut, Resampler};

use std::io::{Read, Write, Seek};

#[derive(Parser)]
#[command(name="audi8", version="1.0", about="A CLI audio transposition tool", long_about = None)]
struct Args {
    #[arg(index=1)]
    file: String,

    #[arg(index=2, allow_hyphen_values = true,
          value_parser = clap::value_parser!(i32).range(-12..=12))]
    num_semitones: i32
}

/// Read samples from a WavReader and split them into channels
fn read_frames<R: Read + Seek>(reader: &mut WavReader<R>, frames: usize) -> Vec<Vec<f32>> {
    let spec = reader.spec();
    let channels = spec.channels as usize;
    let mut waves = vec![Vec::with_capacity(frames); channels];
    let mut samples = reader.samples::<i16>();

    'outer: for _ in 0..frames {
        for c in 0..channels {
            match samples.next().transpose().unwrap() {
                Some(s) => waves[c].push((s as f32) / i16::MAX as f32),
                None => break 'outer,
            }
        }
    }

    waves
}

fn write_frames<W: Write + Seek>(writer: &mut WavWriter<W>, waves: Vec<Vec<f32>>, frames: usize) {
    let channels = waves.len();

    for i in 0..frames {
        for wave in waves.iter().take(channels) {
            let sample = wave[i].clamp(-1.0, 1.0);
            let si16 = (sample * i16::MAX as f32).round() as i16;
            writer.write_sample(si16).unwrap();
        }
    }
}

fn main() {
    let args = Args::parse();

    let file_path = &args.file;
    let pitch_shift = args.num_semitones;

    println!("file path: {file_path}");
    println!("pitch shift: {pitch_shift}");

    let mut reader = match WavReader::open(file_path) {
        Ok(reader) => reader,
        Err(Error::FormatError(..)) => panic!("audio must be a valid WAV file"),
        Err(err) => panic!("could not open audio file: {err}")
    };
    let spec = reader.spec();
    println!("spec: {spec:?}");

    let mut writer = WavWriter::create(
        "output.wav",
        WavSpec {
            channels: spec.channels,
            sample_rate: spec.sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        },
    ).unwrap();

    let chunk_size = 4096usize;
    let sampling_ratio = 2f32.powf(pitch_shift as f32 / 12.0);

    let mut resampler = FftFixedInOut::<f32>::new(
        spec.sample_rate as usize,
        (spec.sample_rate as f32 / sampling_ratio) as usize,
        chunk_size,
        spec.channels as usize,
    ).unwrap();

    loop {
        let in_frames = resampler.input_frames_next();
        let out_frames = resampler.output_frames_next();
        let mut in_block = read_frames(&mut reader, in_frames);
        let exhausted = in_block.iter().any(|ch| ch.len() < in_frames);

        for ch in &mut in_block {
            ch.resize(in_frames, 0.0);
        }

        let out_block = resampler.process(&in_block, None).unwrap();
        write_frames(&mut writer, out_block, out_frames);

        if exhausted {
            break;
        }
    }

    writer.finalize().unwrap();

    println!("successfully resampled {file_path}, saving to output.wav");
}

