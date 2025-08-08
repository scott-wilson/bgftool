mod pcg_noise;
mod r2_blue_noise;

#[derive(Debug, Default, Clone, Copy, clap::ValueEnum)]
pub enum DitherOptions {
    #[default]
    None,
    R2,
    Pcg,
}

pub enum DitherGenerator<const N: usize> {
    None,
    R2(r2_blue_noise::R2BlueNoiseGenerator<N>),
    Pcg(pcg_noise::PcgNoiseGenerator<N>),
}

impl<const N: usize> DitherGenerator<N> {
    pub fn new_none() -> Self {
        Self::None
    }

    pub fn new_r2(seed: f64) -> Self {
        Self::R2(r2_blue_noise::R2BlueNoiseGenerator::new(seed))
    }

    pub fn new_pcg(seed: u64, sample_count: usize) -> Self {
        Self::Pcg(pcg_noise::PcgNoiseGenerator::new(seed, sample_count))
    }

    pub fn from_index(&self, index: usize) -> [f32; N] {
        match self {
            DitherGenerator::None => [0.0; N],
            DitherGenerator::R2(generator) => generator.from_index(index),
            DitherGenerator::Pcg(generator) => generator.from_index(index),
        }
    }
}
