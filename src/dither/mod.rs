use image::{ImageBuffer, Pixel, imageops::ColorMap};
use rayon::prelude::*;

mod pcg_noise;
mod r2_blue_noise;

// Error diffusion dithering based on
// https://tannerhelland.com/2012/12/28/dithering-eleven-algorithms-source-code.html

#[derive(Debug, Default, Clone, Copy, clap::ValueEnum)]
pub enum DitherOptions {
    #[default]
    None,
    R2,
    Pcg,
    FloydSteinberg,
    JavisJudiceNinke,
    Stucki,
    Atkinson,
    Burkes,
    Sierra,
    TwoRowSierra,
    SierraLite,
}

pub enum DitherGenerator {
    None,
    R2(r2_blue_noise::R2BlueNoiseGenerator<4>),
    Pcg(pcg_noise::PcgNoiseGenerator<4>),
    FloydSteinberg,
    JavisJudiceNinke,
    Stucki,
    Atkinson,
    Burkes,
    Sierra,
    TwoRowSierra,
    SierraLite,
}

impl DitherGenerator {
    pub fn new_none() -> Self {
        Self::None
    }

    pub fn new_r2(seed: f64) -> Self {
        Self::R2(r2_blue_noise::R2BlueNoiseGenerator::new(seed))
    }

    pub fn new_pcg(seed: u64, sample_count: usize) -> Self {
        Self::Pcg(pcg_noise::PcgNoiseGenerator::new(seed, sample_count))
    }

    pub fn new_floyd_steinberg() -> Self {
        Self::FloydSteinberg
    }

    pub fn new_javis_judice_ninke() -> Self {
        Self::JavisJudiceNinke
    }

    pub fn new_stucki() -> Self {
        Self::Stucki
    }

    pub fn new_atkinson() -> Self {
        Self::Atkinson
    }

    pub fn new_burkes() -> Self {
        Self::Burkes
    }

    pub fn new_sierra() -> Self {
        Self::Sierra
    }

    pub fn new_two_row_sierra() -> Self {
        Self::TwoRowSierra
    }

    pub fn new_sierra_lite() -> Self {
        Self::SierraLite
    }

    pub fn dither(
        &self,
        image_buffer: &image::ImageBuffer<image::Rgba<f32>, Vec<f32>>,
        options: &crate::bgf::BitmapImageOptions,
        palette: &crate::bgf::Palette,
    ) -> Vec<u8> {
        let (transparent_index, transparent_color) = palette.transparent_color();
        let transparent_color_f = image::Rgb([
            byte_to_float(transparent_color[0]),
            byte_to_float(transparent_color[1]),
            byte_to_float(transparent_color[2]),
        ]);

        match self {
            Self::None => image_buffer
                .par_pixels()
                .map(|pixel| {
                    if pixel[3] < options.transparency_clip || pixel.to_rgb() == transparent_color_f
                    {
                        transparent_index as u8
                    } else {
                        let color_f = pixel.to_rgb();
                        let color = image::Rgb([
                            float_to_byte(color_f[0]),
                            float_to_byte(color_f[1]),
                            float_to_byte(color_f[2]),
                        ]);
                        let color_index = palette.index_of(&color);
                        color_index as u8
                    }
                })
                .collect::<Vec<_>>(),
            Self::R2(r2_blue_noise_generator) => dither_from_noise(
                image_buffer,
                transparent_color_f,
                transparent_index,
                options,
                palette,
                |index| r2_blue_noise_generator.get(index),
            ),
            Self::Pcg(pcg_noise_generator) => dither_from_noise(
                image_buffer,
                transparent_color_f,
                transparent_index,
                options,
                palette,
                |index| pcg_noise_generator.get(index),
            ),
            Self::FloydSteinberg => dither_from_error_diffusion(
                image_buffer,
                transparent_color_f,
                transparent_index,
                options,
                palette,
                &[
                    (7.0 / 16.0, (1, 0)),
                    (3.0 / 16.0, (-1, 1)),
                    (5.0 / 16.0, (0, 1)),
                    (1.0 / 16.0, (1, 1)),
                ],
            ),
            Self::JavisJudiceNinke => dither_from_error_diffusion(
                image_buffer,
                transparent_color_f,
                transparent_index,
                options,
                palette,
                &[
                    (7.0 / 48.0, (1, 0)),
                    (5.0 / 48.0, (2, 0)),
                    (3.0 / 48.0, (-2, 1)),
                    (5.0 / 48.0, (-1, 1)),
                    (7.0 / 48.0, (0, 1)),
                    (5.0 / 48.0, (1, 1)),
                    (3.0 / 48.0, (2, 1)),
                    (1.0 / 48.0, (-2, 2)),
                    (3.0 / 48.0, (-1, 2)),
                    (5.0 / 48.0, (0, 2)),
                    (3.0 / 48.0, (1, 2)),
                    (1.0 / 48.0, (2, 2)),
                ],
            ),
            Self::Stucki => dither_from_error_diffusion(
                image_buffer,
                transparent_color_f,
                transparent_index,
                options,
                palette,
                &[
                    (8.0 / 42.0, (1, 0)),
                    (4.0 / 42.0, (2, 0)),
                    (2.0 / 42.0, (-2, 1)),
                    (4.0 / 42.0, (-1, 1)),
                    (8.0 / 42.0, (0, 1)),
                    (4.0 / 42.0, (1, 1)),
                    (2.0 / 42.0, (2, 1)),
                    (1.0 / 42.0, (-2, 2)),
                    (2.0 / 42.0, (-1, 2)),
                    (4.0 / 42.0, (0, 2)),
                    (2.0 / 42.0, (1, 2)),
                    (1.0 / 42.0, (2, 2)),
                ],
            ),
            Self::Atkinson => dither_from_error_diffusion(
                image_buffer,
                transparent_color_f,
                transparent_index,
                options,
                palette,
                &[
                    (1.0 / 8.0, (1, 0)),
                    (1.0 / 8.0, (2, 0)),
                    (1.0 / 8.0, (-1, 1)),
                    (1.0 / 8.0, (0, 1)),
                    (1.0 / 8.0, (1, 1)),
                    (1.0 / 8.0, (0, 2)),
                ],
            ),
            Self::Burkes => dither_from_error_diffusion(
                image_buffer,
                transparent_color_f,
                transparent_index,
                options,
                palette,
                &[
                    (8.0 / 32.0, (1, 0)),
                    (4.0 / 32.0, (2, 0)),
                    (2.0 / 32.0, (-2, 1)),
                    (4.0 / 32.0, (-1, 1)),
                    (8.0 / 32.0, (0, 1)),
                    (4.0 / 32.0, (1, 1)),
                    (2.0 / 32.0, (2, 1)),
                ],
            ),
            Self::Sierra => dither_from_error_diffusion(
                image_buffer,
                transparent_color_f,
                transparent_index,
                options,
                palette,
                &[
                    (5.0 / 32.0, (1, 0)),
                    (3.0 / 32.0, (2, 0)),
                    (2.0 / 32.0, (-2, 1)),
                    (4.0 / 32.0, (-1, 1)),
                    (5.0 / 32.0, (0, 1)),
                    (4.0 / 32.0, (1, 1)),
                    (2.0 / 32.0, (2, 1)),
                    (2.0 / 32.0, (-1, 2)),
                    (3.0 / 32.0, (0, 2)),
                    (2.0 / 32.0, (1, 2)),
                ],
            ),
            Self::TwoRowSierra => dither_from_error_diffusion(
                image_buffer,
                transparent_color_f,
                transparent_index,
                options,
                palette,
                &[
                    (4.0 / 16.0, (1, 0)),
                    (3.0 / 16.0, (1, 0)),
                    (1.0 / 16.0, (-2, 1)),
                    (2.0 / 16.0, (-1, 1)),
                    (3.0 / 16.0, (0, 1)),
                    (2.0 / 16.0, (1, 1)),
                    (1.0 / 16.0, (2, 1)),
                ],
            ),
            Self::SierraLite => dither_from_error_diffusion(
                image_buffer,
                transparent_color_f,
                transparent_index,
                options,
                palette,
                &[
                    (2.0 / 4.0, (1, 0)),
                    (1.0 / 4.0, (-1, 1)),
                    (1.0 / 4.0, (0, 1)),
                ],
            ),
        }
    }
}

#[inline(always)]
fn float_to_byte(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0) as u8
}

#[inline(always)]
fn byte_to_float(value: u8) -> f32 {
    (value as f32) / 255.0
}

#[inline(always)]
fn propagate_error(
    value: image::Rgba<f32>,
    position: (isize, isize),
    size: (usize, usize),
    buf: &mut [image::Rgba<f32>],
) {
    if position.0 >= size.0 as isize
        || position.1 >= size.1 as isize
        || position.0 <= 0_isize
        || position.1 <= 0_isize
    {
        return;
    }

    let index = (position.1 as usize * size.0) + position.0 as usize;
    let old_error = buf[index];
    let new_error = image::Rgba([
        old_error[0] + value[0],
        old_error[1] + value[1],
        old_error[2] + value[2],
        old_error[3] + value[3],
    ]);
    buf[index] = new_error;
}

fn dither_from_noise<G>(
    image_buffer: &ImageBuffer<image::Rgba<f32>, Vec<f32>>,
    transparent_color_f: image::Rgb<f32>,
    transparent_index: usize,
    options: &crate::bgf::BitmapImageOptions,
    palette: &crate::bgf::Palette,
    generator: G,
) -> Vec<u8>
where
    G: FnOnce(usize) -> [f32; 4] + Sync + Send + Copy,
{
    image_buffer
        .par_pixels()
        .enumerate()
        .map(|(index, pixel)| {
            let noise = generator(index);
            let pixel = {
                let mut pixel = pixel.0;
                pixel
                    .iter_mut()
                    .enumerate()
                    .for_each(|(i, v)| *v += *v * noise[i]);

                image::Rgba(pixel)
            };

            if pixel[3] < options.transparency_clip || pixel.to_rgb() == transparent_color_f {
                transparent_index as u8
            } else {
                let color_f = pixel.to_rgb();
                let color = image::Rgb([
                    float_to_byte(color_f[0]),
                    float_to_byte(color_f[1]),
                    float_to_byte(color_f[2]),
                ]);
                let color_index = palette.index_of(&color);
                color_index as u8
            }
        })
        .collect::<Vec<_>>()
}

fn dither_from_error_diffusion(
    image_buffer: &ImageBuffer<image::Rgba<f32>, Vec<f32>>,
    transparent_color_f: image::Rgb<f32>,
    transparent_index: usize,
    options: &crate::bgf::BitmapImageOptions,
    palette: &crate::bgf::Palette,
    diffusion: &[(f32, (isize, isize))],
) -> Vec<u8> {
    let mut error_buf =
        vec![image::Rgba([0.0f32; 4]); (image_buffer.width() * image_buffer.height()) as usize];

    image_buffer
        .pixels()
        .enumerate()
        .map(|(index, pixel)| {
            let x = index as isize % (image_buffer.width() as isize);
            let y = index as isize / (image_buffer.width() as isize);
            let error = error_buf[index];
            let pixel = image::Rgba([
                pixel[0] + error[0],
                pixel[1] + error[1],
                pixel[2] + error[2],
                pixel[3] + error[3],
            ]);

            let (index, next_color) =
                if pixel[3] < options.transparency_clip || pixel.to_rgb() == transparent_color_f {
                    (transparent_index as u8, image::Rgba([0.0, 0.0, 0.0, 0.0]))
                } else {
                    let color_f = pixel.to_rgb();
                    let color = image::Rgb([
                        float_to_byte(color_f[0]),
                        float_to_byte(color_f[1]),
                        float_to_byte(color_f[2]),
                    ]);
                    let (color_index, next_color) = palette.find_closest(&color);
                    (
                        color_index as u8,
                        image::Rgba([
                            byte_to_float(next_color[0]),
                            byte_to_float(next_color[1]),
                            byte_to_float(next_color[2]),
                            1.0,
                        ]),
                    )
                };
            let diff = image::Rgba([
                pixel[0] - next_color[0],
                pixel[1] - next_color[1],
                pixel[2] - next_color[2],
                pixel[3] - next_color[3],
            ]);

            for (fract, rel_position) in diffusion {
                let position = (rel_position.0 + x, rel_position.1 + y);
                propagate_error(
                    diff.map(|v| v * fract),
                    position,
                    (
                        image_buffer.width() as usize,
                        image_buffer.height() as usize,
                    ),
                    &mut error_buf,
                );
            }

            index
        })
        .collect()
}
