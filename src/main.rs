use clap::Parser;
use hound::{Error, WavReader, WavSpec, WavWriter};

use audi8::{time_scaler::{TimeScaler}, wav};

#[derive(Parser)]
#[command(name="audi8", version="1.0", about="A CLI audio transposition tool", long_about = None)]
struct Args {
    #[arg(index=1)]
    file: String,

    #[arg(index=2, allow_hyphen_values = true,
          value_parser = clap::value_parser!(i32).range(-12..=12))]
    num_semitones: i32
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

    let mut time_scaler = TimeScaler::new(
        chunk_size,
        chunk_size / 2,
        0.5,
        spec.channels as usize,
    );

    // Read entire file into input buffer
    loop {
        let in_block = wav::read_frames(&mut reader, chunk_size);
        let exhausted = in_block.iter().any(|channel| channel.len() < chunk_size);
        time_scaler.push_block(&in_block);

        if exhausted {
            break;
        }
    }

    loop {
        match time_scaler.pop_block() {
            Ok(out_block) => {
                wav::write_frames(&mut writer, out_block, chunk_size);
            }
            Err(..) => break,
        }

    }

    writer.finalize().unwrap();

    println!("successfully time-shifted {file_path}, saving to output.wav");
}

