use serde::Deserialize;
use sqlx::Row;

use crate::models::image::ImageFieldMeta;


// Struct to hold image OCR_TEXT data
#[derive(Debug, Clone, Deserialize)]
pub struct ImageOcrText {
    pub image_path: String,
    pub ocr_text: String,
}

pub const IMAGE_OCR_TEXT_COLUMNS_JSON: &str = r#"
[
    {"name": "image_path", "label": "Image Path", "description": "The file path of the image", "field_type": "string", "example": "/images/photo.jpg", "category": "general"},
    {"name": "ocr_text", "label": "OCR Text", "description": "The text extracted from the image", "field_type": "string", "example": "meow", "category": "general"}
]"#;

pub fn multi_try_get(row: &sqlx::sqlite::SqliteRow, fields: &[&str]) -> Option<String> {
    fields.iter().map(|&field| row.try_get(field).ok()).nth(0).flatten()
}

pub fn multi_try_get_prefixed(row: &sqlx::sqlite::SqliteRow, prefix: &str, field: &str) -> Option<String> {
    multi_try_get(row, &[&format!("{}{}", prefix, field), field])
}

impl ImageOcrText {
    pub fn new(row: &sqlx::sqlite::SqliteRow) -> Self {
        ImageOcrText {
            image_path: row.try_get("image_path").ok().unwrap_or_default(),
            ocr_text: row.try_get("ocr_text").ok().unwrap_or_default(),
        }
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "image_path" => Some(self.image_path.clone()),
            "ocr_text" => Some(self.ocr_text.clone()),
            _ => None,
        }
    }
    
    pub fn get_meta() -> Vec<ImageFieldMeta> {
        serde_json::from_str::<Vec<ImageFieldMeta>>(IMAGE_OCR_TEXT_COLUMNS_JSON).unwrap()
    }
}

impl std::fmt::Display for ImageOcrText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ocr_text: {:?}, ", self.ocr_text)?;
        Ok(())
    }
}