use hound::{WavReader, WavWriter};
use std::io::{Read, Write, Seek};

/// Read samples from a WavReader and split them into channels
pub fn read_frames<R: Read + Seek>(reader: &mut WavReader<R>, frames: usize) -> Vec<Vec<f32>> {
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

pub fn write_frames<W: Write + Seek>(writer: &mut WavWriter<W>, waves: Vec<Vec<f32>>, frames: usize) {
    let channels = waves.len();

    for i in 0..frames {
        for wave in waves.iter().take(channels) {
            let sample = wave[i].clamp(-1.0, 1.0);
            let si16 = (sample * i16::MAX as f32).round() as i16;
            writer.write_sample(si16).unwrap();
        }
    }
}

