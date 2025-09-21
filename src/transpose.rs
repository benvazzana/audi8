use std::io::{Read, Seek, Write};

use hound::{WavReader, WavWriter};
use rubato::{FftFixedInOut, Resampler};

use crate::{error::InsufficientInputError, time_scaler::TimeScaler, wav};


// Helper to transpose the full contents of a WavReader and write the output
// to the provided WavWriter
pub fn transpose_audio<R: Read + Seek, W: Write + Seek>(reader: &mut WavReader<R>,
                                                        writer: &mut WavWriter<W>,
                                                        num_semitones: i32) {
    let spec = reader.spec();
            println!("spec: {spec:?}");

            let chunk_size = 4096usize;
            let sampling_ratio = 2f32.powf(num_semitones as f32 / 12.0);

            let mut resampler = FftFixedInOut::<f32>::new(
                spec.sample_rate as usize,
                (spec.sample_rate as f32 / sampling_ratio) as usize,
                chunk_size,
                spec.channels as usize,
            ).unwrap();

            let mut time_scaler = TimeScaler::new(
                chunk_size,
                chunk_size / 2,
                sampling_ratio,
                spec.channels as usize,
            );

            // Read entire file into input buffer
            loop {
                let in_block = wav::read_frames(reader, chunk_size);
                let exhausted = in_block.iter().any(|channel| channel.len() < chunk_size);
                time_scaler.push_block(&in_block);

                if exhausted {
                    break;
                }
            }

            loop {
                let in_frames = resampler.input_frames_next();
                let out_frames = resampler.output_frames_next();
                match time_scaler.pop_frames(in_frames) {
                    Ok(block) => {
                        let out_block = resampler.process(&block, None).unwrap();
                        wav::write_frames(writer, out_block, out_frames);
                    }
                    Err(InsufficientInputError) => break,
                }

            }
}

