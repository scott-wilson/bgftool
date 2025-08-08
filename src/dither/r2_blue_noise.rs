pub struct R2BlueNoiseGenerator<const N: usize> {
    seed: f64,
    alpha: [f64; N],
}

impl<const N: usize> R2BlueNoiseGenerator<N> {
    pub fn new(seed: f64) -> Self {
        let g = phi(N);
        let mut alpha = [0.0; N];

        alpha
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v = ((1.0 / g).powi(i as i32 + 1)).fract());

        Self { seed, alpha }
    }

    pub fn get(&self, index: usize) -> [f32; N] {
        let mut next_value = [0.0; N];

        next_value
            .iter_mut()
            .zip(self.alpha)
            .enumerate()
            .for_each(|(i, (v, a))| {
                if i != 3 {
                    // Make the RGB channels go from -1.0 to 1.0. This should
                    // make the average brightness of the image the same.
                    *v = ((self.seed + a * (index + 1) as f64).fract() * 2.0 - 1.0) as f32
                } else {
                    // Make the A channels go from 0.0 to 1.0. If we make this
                    // go from -1.0 to 1.0, then it'll potentially make
                    // non-transparent pixels into transparent (when they
                    // shouldn't be).
                    *v = ((self.seed + a * (index + 1) as f64).fract()) as f32
                }
            });

        next_value
    }
}

#[inline(always)]
fn phi(d: usize) -> f64 {
    let mut x = 2.0;

    for _ in 0..10 {
        x = (1.0f64 + x).powf(1.0 / (d + 1) as f64)
    }

    x
}
