use rand::prelude::*;

pub struct PcgNoiseGenerator<const N: usize> {
    samples: Vec<[f32; N]>,
}

impl<const N: usize> PcgNoiseGenerator<N> {
    pub fn new(seed: u64, sample_count: usize) -> Self {
        let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(seed);

        let samples = (0..sample_count)
            .map(|_| {
                let mut sample = [0.0; N];
                sample.iter_mut().enumerate().for_each(|(i, v)| {
                    *v = if i != 3 {
                        rng.random_range(-1.0..1.0)
                    } else {
                        rng.random_range(0.0..1.0)
                    }
                });

                sample
            })
            .collect();

        Self { samples }
    }

    pub fn get(&self, index: usize) -> [f32; N] {
        self.samples[index]
    }
}
