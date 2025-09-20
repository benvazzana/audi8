use crate::{error::InsufficientInputError, window::hann_window};

pub struct TimeScaler {
    block_size: usize,
    hop: usize,
    scaling_factor: f32,
    channels: usize,

    window: Vec<f32>,

    // per-channel input and output
    in_buf: Vec<Vec<f32>>,
    out_buf: Vec<Vec<f32>>,

    next_input_idx: usize,
    next_output_idx: usize,
}
impl TimeScaler {
    pub fn new(block_size: usize, hop: usize, scaling_factor: f32, channels: usize) -> Self {
        Self {
            block_size, hop, scaling_factor, channels,
            window: hann_window(block_size),
            in_buf: vec![Vec::new(); block_size],
            out_buf: vec![Vec::new(); block_size],
            next_input_idx: 0,
            next_output_idx: 0,
        }
    }

    pub fn push_block(&mut self, block: &Vec<Vec<f32>>) {
        for channel in 0..self.channels {
            self.in_buf[channel].extend_from_slice(&block[channel]);
        }
        self.process_available_frames();
    }

    fn process_available_frames(&mut self) {
        while self.next_input_idx + self.block_size <= self.in_buf[0].len() {
            let need = self.next_output_idx + self.block_size;
            for channel in 0..self.channels {
                if self.out_buf[channel].len() < need {
                    self.out_buf[channel].resize(need, 0.0);
                }

                for i in 0..self.block_size {
                    let sample = self.in_buf[channel][self.next_input_idx + i];
                    self.out_buf[channel][self.next_output_idx + i] += sample * self.window[i];
                }
            }
            self.next_input_idx += self.hop;
            self.next_output_idx += (self.hop as f32*self.scaling_factor) as usize;
        }
    }

    pub fn pop_frames(&mut self, frames: usize) -> Result<Vec<Vec<f32>>, InsufficientInputError> {
        if self.next_output_idx < frames {
            return Err(InsufficientInputError);
        }

        let mut block = vec![Vec::with_capacity(frames); self.channels];
        for channel in 0..self.channels {
            for i in 0..frames {
                block[channel].push(self.out_buf[channel][i]);
            }
            self.out_buf[channel].drain(0..frames);
        }
        self.next_output_idx -= frames;
        Ok(block)
    }

}

