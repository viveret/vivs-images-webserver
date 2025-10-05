use serde::Deserialize;
use sqlx::Row;

use crate::models::image::ImageFieldMeta;



#[derive(Debug, Clone, Deserialize)]
pub enum ImageQuality {
    XHD, HD, Med, Low
}

impl TryFrom<u8> for ImageQuality {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Low),
            1 => Ok(Self::Med),
            2 => Ok(Self::HD),
            3 => Ok(Self::XHD),
            _ => Err(())
        }
    }
}

impl TryInto<u8> for &ImageQuality {
    type Error = ();

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            ImageQuality::Low => Ok(0),
            ImageQuality::Med => Ok(1),
            ImageQuality::HD => Ok(2),
            ImageQuality::XHD => Ok(3),
        }
    }
}

// Struct to hold mapping of an image path to calculated aspect_ratio
#[derive(Debug, Clone, Deserialize)]
pub struct ImageAspectRatio {
    pub image_path: String,
    pub width_pixels: u32,
    pub height_pixels: u32,
    pub aspect_ratio: f32,
    pub quality: ImageQuality,
}

pub const IMAGE_ASPECT_RATIO_COLUMNS_JSON: &str = r#"
[
    {"name": "image_path", "label": "Image Path", "description": "The file path of the image", "field_type": "string", "example": "/images/photo.jpg", "category": "general", "table_name": "image_aspect_ratio"},
    {"name": "width_pixels", "label": "Width (pixels)", "description": "The width of the image in pixels", "field_type": "u32", "example":"64", "category": "general", "table_name": "image_aspect_ratio"},
    {"name": "height_pixels", "label": "Height (pixels)", "description": "The height of the image in pixels", "field_type": "u32", "example":"64", "category": "general", "table_name": "image_aspect_ratio"},
    {"name": "aspect_ratio", "label": "Aspect Ratio", "description": "The aspect ratio of the image", "field_type": "f32", "example":"0.4", "category": "general", "table_name": "image_aspect_ratio"},
    {"name": "quality", "label": "Quality", "description": "The quality of the image", "field_type": "u8", "example":"1", "category": "general", "table_name": "image_aspect_ratio"}
]"#;

impl ImageAspectRatio {
    pub fn new(row: &sqlx::sqlite::SqliteRow) -> Self {
        let image_path: String = row.try_get("image_path").unwrap_or_default();
        let width_pixels: u32 = row.try_get("width_pixels").unwrap_or_default();
        let height_pixels: u32 = row.try_get("height_pixels").unwrap_or_default();
        let aspect_ratio: f32 = row.try_get("aspect_ratio").unwrap_or(0.0);

        let quality: u8 = row.try_get("quality").unwrap_or_default();
        let quality = quality.try_into().unwrap();
        
        ImageAspectRatio {
            image_path,
            width_pixels,
            height_pixels,
            aspect_ratio,
            quality
        }
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "image_path" => Some(self.image_path.clone()),
            "width_pixels" => Some(format!("{}", self.width_pixels)),
            "height_pixels" => Some(format!("{}", self.height_pixels)),
            "aspect_ratio" => Some(format!("{:.2}", self.aspect_ratio)),
            "quality" => {
                let q: u8 = (&self.quality).try_into().unwrap();
                Some(format!("{}", q))
            },
            _ => None,
        }
    }
    
    pub fn get_meta() -> Vec<ImageFieldMeta> {
        serde_json::from_str::<Vec<ImageFieldMeta>>(IMAGE_ASPECT_RATIO_COLUMNS_JSON).unwrap()
    }
}

impl std::fmt::Display for ImageAspectRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "aspect_ratio: {}", self.aspect_ratio)
    }
}