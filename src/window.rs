pub fn hann_window(n: usize) -> Vec<f32> {
    (0..n).map(|i| {
        let x = std::f32::consts::PI * 2.0 * (i as f32) / (n as f32);
        0.5 * (1.0 - x.cos())
    }).collect()
}

