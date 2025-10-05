use serde::Deserialize;
use sqlx::Row;

use crate::models::image::ImageFieldMeta;

// Struct to hold mapping of an image path to extracted xmp data
#[derive(Debug, Clone, Deserialize)]
pub struct ImageXmp {
    pub image_path: String,
    pub xmp: String,
}

pub const IMAGE_XMP_COLUMNS_JSON: &str = r#"
[
    {"name": "image_path", "label": "Image Path", "description": "The file path of the image", "field_type": "string", "example": "/images/photo.jpg", "category": "general", "table_name": "image_xmp"},
    {"name": "xmp", "label": "Xmp", "description": "The xmp data of the image", "field_type": "string", "example":"no example", "category": "general", "table_name": "image_xmp"}
]"#;

impl ImageXmp {
    pub fn new(row: &sqlx::sqlite::SqliteRow) -> Self {
        let image_path: String = row.try_get("image_path").unwrap_or_default();
        let xmp: String = row.try_get("xmp").unwrap_or_default();
        ImageXmp {
            image_path,
            xmp,
        }
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "image_path" => Some(self.image_path.clone()),
            "xmp" => Some(self.xmp.clone()),
            _ => None,
        }
    }
    
    pub fn get_meta() -> Vec<ImageFieldMeta> {
        serde_json::from_str::<Vec<ImageFieldMeta>>(IMAGE_XMP_COLUMNS_JSON).unwrap()
    }
}

impl std::fmt::Display for ImageXmp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "xmp: {}", self.xmp)
    }
}