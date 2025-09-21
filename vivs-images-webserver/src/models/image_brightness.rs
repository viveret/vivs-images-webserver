use sqlx::Row;

use crate::models::image::ImageFieldMeta;


pub struct ImageBrightness {
    pub image_path: String,
    pub brightness: f32,
}

pub const IMAGE_BRIGHTNESS_COLUMNS_JSON: &str = r#"
[
    {"name": "image_path", "label": "Image Path", "description": "The file path of the image", "field_type": "string", "example": "/images/photo.jpg", "category": "general"},
    {"name": "brightness", "label": "Brightness", "description": "The brightness of the image", "field_type": "f32", "example":"0.4", "category": "general"}
]"#;

impl ImageBrightness {
    pub fn new(row: &sqlx::sqlite::SqliteRow) -> Self {
        let image_path: String = row.try_get("image_path").unwrap_or_default();
        let brightness: f32 = row.try_get("brightness").unwrap_or(0.0);
        ImageBrightness {
            image_path,
            brightness,
        }
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "brightness" => Some(format!("{:.2}", self.brightness)),
            _ => None,
        }
    }
    
    pub fn get_meta() -> Vec<ImageFieldMeta> {
        serde_json::from_str::<Vec<ImageFieldMeta>>(IMAGE_BRIGHTNESS_COLUMNS_JSON).unwrap()
    }
}
