use std::{io::prelude::*, str::FromStr};

use color_eyre::eyre::{self, Ok, Result};
use rayon::prelude::*;

const MAGIC_NUMBER: &[u8] = b"BGF\x11";
const CURRENT_BGF_VERSION: i32 = 10;
const MAX_BITMAP_NAME_LEN: usize = 32;
const PALETTE: &[[u8; 3]] = &[
    [0, 0, 0],
    [128, 0, 0],
    [0, 128, 0],
    [128, 128, 0],
    [0, 0, 128],
    [128, 0, 128],
    [0, 128, 128],
    [192, 192, 192],
    [128, 0, 0],
    [0, 128, 0],
    [128, 0, 0],
    [0, 128, 0],
    [128, 0, 0],
    [0, 128, 0],
    [128, 0, 0],
    [0, 128, 0],
    [194, 1, 1],
    [180, 1, 1],
    [171, 2, 2],
    [166, 1, 1],
    [154, 2, 2],
    [145, 2, 0],
    [137, 2, 0],
    [127, 0, 0],
    [120, 2, 0],
    [109, 1, 0],
    [86, 0, 0],
    [76, 0, 0],
    [64, 0, 0],
    [56, 0, 0],
    [38, 0, 0],
    [17, 0, 0],
    [254, 194, 148],
    [235, 184, 146],
    [219, 169, 131],
    [203, 157, 124],
    [198, 148, 117],
    [181, 135, 105],
    [177, 136, 102],
    [168, 128, 96],
    [157, 115, 86],
    [145, 107, 81],
    [136, 96, 72],
    [122, 88, 68],
    [117, 84, 64],
    [104, 77, 59],
    [96, 70, 49],
    [74, 59, 45],
    [255, 181, 128],
    [243, 168, 114],
    [220, 153, 104],
    [202, 141, 97],
    [196, 130, 87],
    [185, 122, 81],
    [171, 115, 71],
    [165, 110, 68],
    [147, 92, 54],
    [133, 82, 49],
    [123, 70, 38],
    [107, 61, 34],
    [99, 56, 28],
    [85, 47, 24],
    [75, 40, 13],
    [50, 28, 11],
    [185, 95, 43],
    [145, 70, 26],
    [131, 63, 24],
    [121, 59, 22],
    [119, 52, 18],
    [114, 47, 16],
    [105, 48, 12],
    [102, 45, 12],
    [94, 37, 12],
    [84, 34, 12],
    [75, 27, 11],
    [65, 25, 11],
    [60, 23, 11],
    [51, 20, 11],
    [42, 20, 11],
    [27, 15, 10],
    [255, 178, 51],
    [255, 169, 27],
    [255, 165, 17],
    [250, 156, 0],
    [238, 148, 0],
    [216, 135, 0],
    [204, 127, 0],
    [194, 121, 0],
    [170, 106, 0],
    [160, 100, 0],
    [136, 85, 0],
    [126, 79, 0],
    [104, 65, 0],
    [92, 57, 0],
    [68, 42, 0],
    [48, 30, 0],
    [137, 177, 116],
    [130, 169, 110],
    [120, 161, 100],
    [112, 149, 92],
    [103, 139, 83],
    [95, 129, 76],
    [88, 124, 73],
    [80, 112, 66],
    [71, 101, 55],
    [62, 90, 49],
    [48, 79, 38],
    [41, 68, 31],
    [37, 62, 22],
    [28, 48, 16],
    [16, 30, 8],
    [7, 14, 3],
    [0, 196, 50],
    [0, 184, 47],
    [0, 170, 43],
    [0, 158, 39],
    [0, 154, 39],
    [0, 140, 36],
    [0, 138, 35],
    [0, 126, 32],
    [0, 114, 29],
    [0, 98, 25],
    [0, 80, 20],
    [0, 69, 17],
    [0, 62, 16],
    [0, 48, 12],
    [0, 26, 7],
    [0, 14, 4],
    [171, 213, 222],
    [165, 206, 215],
    [137, 188, 197],
    [127, 172, 179],
    [112, 154, 163],
    [106, 145, 154],
    [78, 129, 137],
    [72, 117, 125],
    [52, 95, 103],
    [46, 85, 93],
    [27, 70, 78],
    [23, 61, 70],
    [10, 52, 61],
    [6, 41, 48],
    [3, 27, 33],
    [0, 9, 11],
    [52, 78, 222],
    [50, 74, 211],
    [43, 62, 199],
    [42, 58, 188],
    [36, 52, 171],
    [34, 48, 161],
    [27, 44, 146],
    [23, 38, 132],
    [10, 27, 120],
    [8, 24, 107],
    [2, 18, 86],
    [1, 15, 75],
    [0, 10, 70],
    [0, 7, 59],
    [0, 3, 41],
    [0, 0, 24],
    [160, 66, 194],
    [153, 63, 185],
    [148, 56, 178],
    [134, 46, 162],
    [122, 44, 161],
    [110, 40, 147],
    [102, 36, 139],
    [94, 32, 129],
    [86, 24, 111],
    [78, 18, 99],
    [63, 3, 85],
    [54, 0, 76],
    [45, 0, 62],
    [33, 0, 47],
    [23, 0, 32],
    [10, 0, 16],
    [244, 240, 206],
    [237, 231, 176],
    [235, 228, 163],
    [229, 220, 137],
    [216, 215, 246],
    [187, 186, 240],
    [175, 173, 237],
    [148, 145, 231],
    [156, 233, 156],
    [132, 228, 132],
    [90, 215, 90],
    [40, 184, 40],
    [242, 197, 197],
    [232, 152, 152],
    [225, 119, 119],
    [220, 98, 98],
    [255, 234, 110],
    [250, 222, 55],
    [247, 213, 27],
    [240, 208, 25],
    [238, 202, 26],
    [222, 189, 25],
    [220, 196, 19],
    [207, 185, 16],
    [197, 180, 10],
    [185, 167, 8],
    [154, 137, 2],
    [135, 122, 0],
    [128, 115, 0],
    [119, 113, 0],
    [112, 106, 0],
    [85, 81, 0],
    [231, 231, 231],
    [213, 213, 213],
    [205, 205, 205],
    [188, 188, 188],
    [180, 180, 180],
    [163, 163, 163],
    [154, 154, 154],
    [146, 146, 146],
    [129, 129, 129],
    [120, 120, 120],
    [103, 103, 103],
    [95, 95, 95],
    [78, 78, 78],
    [70, 70, 70],
    [52, 52, 52],
    [36, 36, 36],
    [124, 191, 255],
    [103, 171, 239],
    [95, 163, 231],
    [95, 154, 213],
    [78, 137, 197],
    [70, 120, 171],
    [61, 112, 163],
    [60, 107, 154],
    [52, 95, 137],
    [44, 82, 119],
    [27, 65, 103],
    [17, 47, 77],
    [10, 36, 61],
    [5, 24, 43],
    [1, 14, 27],
    [0, 11, 22],
    [224, 180, 148],
    [208, 176, 132],
    [204, 168, 124],
    [196, 160, 116],
    [128, 0, 0],
    [0, 128, 0],
    [128, 0, 0],
    [0, 128, 0],
    [128, 128, 128],
    [255, 0, 0],
    [0, 255, 0],
    [255, 255, 0],
    [0, 0, 255],
    [255, 0, 255],
    [0, 255, 255],
    [255, 255, 255],
];

#[derive(Debug)]
pub struct Point(pub i32, pub i32);

#[derive(Debug)]
pub struct Hotspot {
    pub number: i8,
    pub position: Point,
}

impl Hotspot {
    pub fn read<R: Read>(mut reader: R) -> Result<Self> {
        // Extract number
        let mut number_bytes = [0u8; 1];
        reader.read_exact(&mut number_bytes)?;
        let number = i8::from_ne_bytes(number_bytes);

        // Extract X hotspot
        let mut hotspot_x_bytes = [0u8; 4];
        reader.read_exact(&mut hotspot_x_bytes)?;
        let hotspot_x = i32::from_ne_bytes(hotspot_x_bytes);

        // Extract Y hotspot
        let mut hotspot_y_bytes = [0u8; 4];
        reader.read_exact(&mut hotspot_y_bytes)?;
        let hotspot_y = i32::from_ne_bytes(hotspot_y_bytes);

        Ok(Self {
            number,
            position: Point(hotspot_x, hotspot_y),
        })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> Result<()> {
        writer.write_all(&self.number.to_ne_bytes())?;
        writer.write_all(&self.position.0.to_ne_bytes())?;
        writer.write_all(&self.position.1.to_ne_bytes())?;

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct BitmapImageOptions {
    pub compression: crate::conf::BitmapDataCompression,
    pub transparency_clip: f32,
    pub dither: crate::dither::DitherOptions,
}

#[derive(Debug)]
pub enum BitmapData {
    Uncompressed(Vec<u8>),
    ZlibCompressed(Vec<u8>),
}

impl BitmapData {
    pub fn read<R: Read>(mut reader: R) -> Result<Self> {
        // Extract compression
        let mut compression_bytes = [0u8; 1];
        reader.read_exact(&mut compression_bytes)?;
        let compression = u8::from_ne_bytes(compression_bytes);

        // Extract data length
        let mut data_len_bytes = [0u8; 4];
        reader.read_exact(&mut data_len_bytes)?;
        let data_len = i32::from_ne_bytes(data_len_bytes);

        let mut raw_data = vec![0u8; data_len as usize];
        reader.read_exact(&mut raw_data)?;

        let data = match compression {
            0 => Self::Uncompressed(raw_data),
            1 => Self::ZlibCompressed(raw_data),
            _ => return Err(eyre::eyre!("Invalid compression")),
        };

        Ok(data)
    }

    pub fn write<W: Write>(&self, mut writer: W) -> Result<()> {
        let (compression, data) = match self {
            Self::Uncompressed(items) => (0u8, items),
            Self::ZlibCompressed(items) => (1u8, items),
        };

        writer.write_all(&compression.to_ne_bytes())?;
        writer.write_all(&(data.len() as i32).to_ne_bytes())?;
        writer.write_all(data)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Bitmap {
    pub size: (i32, i32),
    pub offset: (i32, i32),
    pub hotspots: Vec<Hotspot>,
    pub data: BitmapData,
}

impl Bitmap {
    pub fn read<R: Read>(mut reader: R) -> Result<Self> {
        // Extract width
        let mut width_bytes = [0u8; 4];
        reader.read_exact(&mut width_bytes)?;
        let width = i32::from_ne_bytes(width_bytes);
        // Extract height
        let mut height_bytes = [0u8; 4];
        reader.read_exact(&mut height_bytes)?;
        let height = i32::from_ne_bytes(height_bytes);

        // Extract X offset
        let mut offset_x_bytes = [0u8; 4];
        reader.read_exact(&mut offset_x_bytes)?;
        let offset_x = i32::from_ne_bytes(offset_x_bytes);
        // Extract Y offset
        let mut offset_y_bytes = [0u8; 4];
        reader.read_exact(&mut offset_y_bytes)?;
        let offset_y = i32::from_ne_bytes(offset_y_bytes);

        // Extract hotspot count
        let mut hotspot_count_bytes = [0u8; 1];
        reader.read_exact(&mut hotspot_count_bytes)?;
        let hotspot_count = u8::from_ne_bytes(hotspot_count_bytes);

        // Extract hotspots
        let mut hotspots = Vec::with_capacity(hotspot_count as usize);

        for _ in 0..hotspot_count {
            hotspots.push(Hotspot::read(&mut reader)?);
        }

        let data = BitmapData::read(&mut reader)?;

        Ok(Self {
            size: (width, height),
            offset: (offset_x, offset_y),
            hotspots,
            data,
        })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> Result<()> {
        let width_bytes = self.size.0.to_ne_bytes();
        let height_bytes = self.size.1.to_ne_bytes();

        // Write out bitmap size
        writer.write_all(&width_bytes)?;
        writer.write_all(&height_bytes)?;

        // Write out x and y offsets
        writer.write_all(&self.offset.0.to_ne_bytes())?;
        writer.write_all(&self.offset.1.to_ne_bytes())?;

        // Write out hotspots
        writer.write_all(&(self.hotspots.len() as u8).to_ne_bytes())?;

        for hotspot in &self.hotspots {
            hotspot.write(&mut writer)?;
        }

        // Write out the bytes of the bitmap
        self.data.write(writer)?;

        Ok(())
    }

    pub fn save_image<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        let data = match &self.data {
            BitmapData::Uncompressed(items) => std::borrow::Cow::Borrowed(items),
            BitmapData::ZlibCompressed(items) => {
                let mut decoder = flate2::read::ZlibDecoder::new(&items[..]);
                let mut data = Vec::new();
                decoder.read_to_end(&mut data)?;
                std::borrow::Cow::Owned(data)
            }
        };

        let mut img = image::RgbImage::new(self.size.0 as u32, self.size.1 as u32);

        for (index, palette_index) in data.iter().enumerate() {
            let pixel = PALETTE[*palette_index as usize];
            let x = index as u32 % self.size.0 as u32;
            let y = index as u32 / self.size.0 as u32;

            img.put_pixel(x, y, image::Rgb(pixel));
        }

        img.save(path)?;

        Ok(())
    }

    pub fn from_image<P: AsRef<std::path::Path>>(
        path: P,
        options: &BitmapImageOptions,
    ) -> Result<Self> {
        let img = image::ImageReader::open(path)?
            .with_guessed_format()?
            .decode()?;
        let width = img.width();
        let height = img.height();
        let palette = Palette::new();

        let image_buffer = img.into_rgba32f();

        let generator = match options.dither {
            crate::dither::DitherOptions::None => crate::dither::DitherGenerator::new_none(),
            crate::dither::DitherOptions::R2 => crate::dither::DitherGenerator::new_r2(0.0),
            crate::dither::DitherOptions::Pcg => {
                crate::dither::DitherGenerator::new_pcg(0, (width * height) as usize)
            }
            crate::dither::DitherOptions::FloydSteinberg => {
                crate::dither::DitherGenerator::new_floyd_steinberg()
            }
            crate::dither::DitherOptions::JavisJudiceNinke => {
                crate::dither::DitherGenerator::new_javis_judice_ninke()
            }
            crate::dither::DitherOptions::Stucki => crate::dither::DitherGenerator::new_stucki(),
            crate::dither::DitherOptions::Atkinson => {
                crate::dither::DitherGenerator::new_atkinson()
            }
            crate::dither::DitherOptions::Burkes => crate::dither::DitherGenerator::new_burkes(),
            crate::dither::DitherOptions::Sierra => crate::dither::DitherGenerator::new_sierra(),
            crate::dither::DitherOptions::TwoRowSierra => {
                crate::dither::DitherGenerator::new_two_row_sierra()
            }
            crate::dither::DitherOptions::SierraLite => {
                crate::dither::DitherGenerator::new_sierra_lite()
            }
        };

        let buf = generator.dither(&image_buffer, options, &palette);
        let data = match options.compression {
            crate::conf::BitmapDataCompression::Uncompressed => BitmapData::Uncompressed(buf),
            crate::conf::BitmapDataCompression::ZlibCompressed => {
                let mut encoder =
                    flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::best());
                encoder.write_all(&buf)?;
                BitmapData::ZlibCompressed(encoder.finish()?)
            }
        };

        Ok(Self {
            size: (width as i32, height as i32),
            offset: (0, 0),
            hotspots: Vec::new(),
            data,
        })
    }
}

#[derive(Debug)]
pub struct Group {
    pub indices: Vec<i32>,
}

impl Group {
    pub fn read<R: Read>(mut reader: R) -> Result<Self> {
        // Extract indices count
        let mut indices_count_bytes = [0u8; 4];
        reader.read_exact(&mut indices_count_bytes)?;
        let indices_count = i32::from_ne_bytes(indices_count_bytes);

        let mut indices = Vec::with_capacity(indices_count as usize);

        for _ in 0..indices_count {
            // Extract index
            let mut index_bytes = [0u8; 4];
            reader.read_exact(&mut index_bytes)?;
            let index = i32::from_ne_bytes(index_bytes);
            indices.push(index);
        }

        Ok(Self { indices })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> Result<()> {
        writer.write_all(&(self.indices.len() as i32).to_ne_bytes())?;

        for index in &self.indices {
            writer.write_all(&index.to_ne_bytes())?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Bgf {
    pub version: i32,
    pub name: String,
    pub bitmaps: Vec<Bitmap>,
    pub index_groups: Vec<Group>,
    pub shrink_factor: i32,
}

impl Bgf {
    pub fn read<R: Read>(mut reader: R) -> Result<Self> {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;

        if magic != MAGIC_NUMBER {
            return Err(eyre::eyre!("Magic number is invalid."));
        }

        // Extract version
        let mut version_bytes = [0u8; 4];
        reader.read_exact(&mut version_bytes)?;
        let version = i32::from_ne_bytes(version_bytes);

        // Extract name
        let mut name_bytes = [0u8; MAX_BITMAP_NAME_LEN];
        reader.read_exact(&mut name_bytes)?;
        let c_name = std::ffi::CStr::from_bytes_until_nul(&name_bytes)?;
        let name = c_name.to_string_lossy().to_string();

        // Extract bitmap count
        let mut bitmap_count_bytes = [0u8; 4];
        reader.read_exact(&mut bitmap_count_bytes)?;
        let bitmap_count = i32::from_ne_bytes(bitmap_count_bytes);

        // Extract index group count
        let mut index_group_count_bytes = [0u8; 4];
        reader.read_exact(&mut index_group_count_bytes)?;
        let index_group_count = i32::from_ne_bytes(index_group_count_bytes);

        // Extract max number of indices in a group
        let mut max_indices_count_bytes = [0u8; 4];
        reader.read_exact(&mut max_indices_count_bytes)?;
        let max_indices = i32::from_ne_bytes(max_indices_count_bytes);

        // Extract shrink factor
        let mut shrink_factor_bytes = [0u8; 4];
        reader.read_exact(&mut shrink_factor_bytes)?;
        let shrink_factor = i32::from_ne_bytes(shrink_factor_bytes);

        // Extract bitmaps
        let mut bitmaps = Vec::with_capacity(bitmap_count as usize);

        for _ in 0..bitmap_count {
            bitmaps.push(Bitmap::read(&mut reader)?);
        }

        // Extract index groups
        let mut index_groups = Vec::with_capacity(index_group_count as usize);

        for _ in 0..index_group_count {
            let index_group = Group::read(&mut reader)?;
            for index in &index_group.indices {
                if *index > max_indices {
                    return Err(eyre::eyre!(
                        "Found index that is greater than the max indices."
                    ));
                }
            }

            index_groups.push(index_group);
        }

        Ok(Self {
            version,
            name,
            bitmaps,
            index_groups,
            shrink_factor,
        })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> Result<()> {
        // Write magic number
        writer.write_all(MAGIC_NUMBER)?;

        // Write version
        writer.write_all(&CURRENT_BGF_VERSION.to_ne_bytes())?;

        // Write bitmap name
        let c_name = std::ffi::CString::from_str(&self.name)?;
        let mut name_bytes = [0u8; MAX_BITMAP_NAME_LEN];

        if c_name.as_bytes_with_nul().len() > MAX_BITMAP_NAME_LEN {
            return Err(eyre::eyre!("BGF name is too large."));
        }

        for (i, c) in c_name.as_bytes_with_nul().iter().enumerate() {
            name_bytes[i] = *c;
        }

        writer.write_all(&name_bytes)?;

        // Write number of bitmaps
        writer.write_all(&(self.bitmaps.len() as i32).to_ne_bytes())?;

        // Write number of index groups
        writer.write_all(&(self.index_groups.len() as i32).to_ne_bytes())?;

        // Find most indices in a group
        let max_indices = self
            .index_groups
            .iter()
            .map(|i| i.indices.len())
            .max()
            .unwrap_or_default();
        writer.write_all(&(max_indices as i32).to_ne_bytes())?;

        // Write shrink factor
        writer.write_all(&self.shrink_factor.to_ne_bytes())?;

        // Write out bitmaps
        for bitmap in &self.bitmaps {
            bitmap.write(&mut writer)?;
        }

        // Write out indices
        for group in &self.index_groups {
            group.write(&mut writer)?;
        }

        Ok(())
    }
}

pub struct Palette {
    values: &'static [image::Rgb<u8>],
}

impl Default for Palette {
    fn default() -> Self {
        Self::new()
    }
}

impl Palette {
    pub fn new() -> Self {
        static CACHED_PALETTE: std::sync::LazyLock<Vec<image::Rgb<u8>>> =
            std::sync::LazyLock::new(|| PALETTE.iter().map(|v| image::Rgb(*v)).collect());

        Self {
            values: CACHED_PALETTE.as_ref(),
        }
    }

    pub fn transparent_color(&self) -> (usize, image::Rgb<u8>) {
        (254, image::Rgb([0, 255, 255]))
    }

    pub fn values(&self) -> &[image::Rgb<u8>] {
        self.values
    }

    pub fn find_closest(&self, color: &image::Rgb<u8>) -> (usize, &image::Rgb<u8>) {
        self.values
            .par_iter()
            .enumerate()
            .filter(|i| i.0 != self.transparent_color().0) // Skip the transparent color
            .min_by(|(_, a), (_, b)| {
                let a = [a[0] as f32, a[1] as f32, a[2] as f32];
                let b = [b[0] as f32, b[1] as f32, b[2] as f32];
                let c = [color[0] as f32, color[1] as f32, color[2] as f32];

                let aa =
                    ((a[0] - c[0]).powi(2) + (a[1] - c[1]).powi(2) + (a[2] - c[2]).powi(2)).sqrt();
                let bb =
                    ((b[0] - c[0]).powi(2) + (b[1] - c[1]).powi(2) + (b[2] - c[2]).powi(2)).sqrt();

                aa.partial_cmp(&bb).unwrap()
            })
            .unwrap()
    }
}

impl image::imageops::ColorMap for Palette {
    type Color = image::Rgb<u8>;

    fn index_of(&self, color: &Self::Color) -> usize {
        let (index, _) = self.find_closest(color);

        index
    }

    fn map_color(&self, color: &mut Self::Color) {
        let (_, closest_color) = self.find_closest(color);

        *color = *closest_color;
    }
}
