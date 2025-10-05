use std::cmp::max;

use image::DynamicImage;
use sqlx::Row;

use crate::converters::extract_image_thumbnail::convert_vec_u8_to_image;
use crate::models::image::ImageFieldMeta;
use crate::converters::extract_image_thumbnail::ExtractImageThumbnailOptions;
use crate::converters::extract_image_thumbnail::convert_image_to_vec_u8;


#[derive(Clone, Copy, Debug)]
pub enum ThumbnailFormat { 
    PNG,
    JPG,
    BMP,
    RAW_rgb8,
}

impl TryFrom<u8> for ThumbnailFormat {
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PNG),
            1 => Ok(Self::JPG),
            2 => Ok(Self::BMP),
            3 => Ok(Self::RAW_rgb8),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
        }
    }
    
    type Error = std::io::Error;
}

impl TryInto<u8> for &ThumbnailFormat {
    type Error = std::io::Error;

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            ThumbnailFormat::PNG => Ok(0),
            ThumbnailFormat::JPG => Ok(1),
            ThumbnailFormat::BMP => Ok(2),
            ThumbnailFormat::RAW_rgb8 => Ok(3),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
        }
    }
}

impl std::fmt::Display for ThumbnailFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: u8 = self.try_into().unwrap();
        f.write_str(&format!("{}", v))
    }
}

#[derive(Clone, Debug)]
pub struct ImageThumbnail {
    pub image_path: String,
    pub width_and_length: u32,
    pub thumbnail_format: ThumbnailFormat,
    pub thumbnail_data: Vec<u8>,
}

impl std::fmt::Display for ImageThumbnail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "width_and_length: {}, ", self.width_and_length)?;
        write!(f, "thumbnail_format: {}", self.thumbnail_format)?;
        Ok(())
    }
}

pub const IMAGE_THUMBNAIL_COLUMNS_JSON: &str = r#"
[
    {"name": "image_path", "label": "Image Path", "description": "The file path of the image", "field_type": "string", "example": "/images/photo.jpg", "category": "general", "table_name": "image_thumbnail"},
    {"name": "width_and_length", "label": "Width and Length", "description": "The width and length of the thumbnail image", "field_type": "integer", "example": "64", "category": "general", "table_name": "image_thumbnail"},
    {"name": "thumbnail_format", "label": "Thumbnail Format", "description": "The format of the thumbnail image", "field_type": "integer", "example": "1", "category": "general", "table_name": "image_thumbnail"},
    {"name": "thumbnail_data", "label": "Data", "description": "The data of the thumbnail image", "field_type": "blob", "category": "general", "table_name": "image_thumbnail"}
]"#;

impl ImageThumbnail {
    pub fn get_meta() -> Vec<ImageFieldMeta> {
        serde_json::from_str::<Vec<ImageFieldMeta>>(IMAGE_THUMBNAIL_COLUMNS_JSON).unwrap()
    }

    pub fn new(
        image_path: String,
        width_and_length: u32,
        thumbnail_format: ThumbnailFormat,
        thumbnail_data: Vec<u8>,
    ) -> Self {
        Self {
            image_path,
            width_and_length,
            thumbnail_format,
            thumbnail_data
        }
    }

    pub fn from_image(
        image_path: String,
        thumbnail_format: ThumbnailFormat,
        thumbnail_image: &DynamicImage,
    ) -> Self {
        let width_and_length = max(thumbnail_image.width(), thumbnail_image.height());
        let options = ExtractImageThumbnailOptions {
            filter: image::imageops::FilterType::Lanczos3,
            output_format: thumbnail_format,
            width_and_length,
        };
        let thumbnail_data = convert_image_to_vec_u8(thumbnail_image, &options).unwrap();
        Self {
            image_path,
            width_and_length,
            thumbnail_format,
            thumbnail_data
        }
    }

    pub fn get_field(&self, c: &str) -> Option<String> {
        match c {
            "image_path" => Some(self.image_path.clone()),
            "width_and_length" => Some(self.width_and_length.to_string()),
            "thumbnail_format" => Some(self.thumbnail_format.to_string()),
            _ => None
        }
    }
    
    pub fn get_field_blob(&self, c: &str) -> Option<Vec<u8>> {
        if c == "thumbnail_data" {
            Some(self.thumbnail_data.clone())
        } else {
            None
        }
    }
    
    pub fn new_from_row(row: &sqlx::sqlite::SqliteRow) -> std::io::Result<Self> {
        let wrap = |row: &sqlx::sqlite::SqliteRow| -> sqlx::Result<Self> {
            Ok(Self {
                image_path: row.try_get("image_path")?,
                width_and_length: row.try_get("width_and_length")?,
                thumbnail_format: ThumbnailFormat::try_from(row.try_get::<i32, _>("thumbnail_format")? as u8)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
                thumbnail_data: row.try_get::<Vec<u8>, _>("thumbnail_data")?,
            })
        };
        wrap(row).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
    
    pub fn to_image(&self) -> std::io::Result<DynamicImage> {
        convert_vec_u8_to_image(&self.thumbnail_data)
    }
}



pub struct ImageThumbnailVec(pub Vec<ImageThumbnail>);

impl std::fmt::Display for ImageThumbnailVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_as_strs: Vec<String> = self.0.iter().map(|x| format!("{}", x)).collect();
        let self_as_str = self_as_strs.join(", ");
        write!(f, "{}", self_as_str)
    }
}