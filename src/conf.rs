#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Point(pub i32, pub i32);

impl From<crate::bgf::Point> for Point {
    fn from(value: crate::bgf::Point) -> Self {
        Self(value.0, value.1)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Hotspot {
    pub number: i8,
    pub position: Point,
}

impl From<crate::bgf::Hotspot> for Hotspot {
    fn from(value: crate::bgf::Hotspot) -> Self {
        Self {
            number: value.number,
            position: value.position.into(),
        }
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub enum BitmapDataCompression {
    #[default]
    #[serde(rename = "none")]
    Uncompressed,
    #[serde(rename = "zlib")]
    ZlibCompressed,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Bitmap {
    pub size: (i32, i32),
    pub offset: (i32, i32),
    pub hotspots: Vec<Hotspot>,
    pub compression: BitmapDataCompression,
    pub path: std::path::PathBuf,
}

impl From<crate::bgf::Bitmap> for Bitmap {
    fn from(value: crate::bgf::Bitmap) -> Self {
        let compression = match value.data {
            crate::bgf::BitmapData::Uncompressed(_) => BitmapDataCompression::Uncompressed,
            crate::bgf::BitmapData::ZlibCompressed(_) => BitmapDataCompression::ZlibCompressed,
        };

        Self {
            size: value.size,
            offset: value.offset,
            hotspots: value.hotspots.into_iter().map(|h| h.into()).collect(),
            compression,
            path: std::path::PathBuf::new(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Group {
    pub indices: Vec<i32>,
}

impl From<crate::bgf::Group> for Group {
    fn from(value: crate::bgf::Group) -> Self {
        Self {
            indices: value.indices,
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Bgf {
    pub version: i32,
    pub name: String,
    pub bitmaps: Vec<Bitmap>,
    pub index_groups: Vec<Group>,
    pub max_indices: i32,
    pub shrink_factor: i32,
}

impl From<crate::bgf::Bgf> for Bgf {
    fn from(value: crate::bgf::Bgf) -> Self {
        Self {
            version: value.version,
            name: value.name,
            bitmaps: value.bitmaps.into_iter().map(|b| b.into()).collect(),
            index_groups: value.index_groups.into_iter().map(|g| g.into()).collect(),
            max_indices: value.max_indices,
            shrink_factor: value.shrink_factor,
        }
    }
}
