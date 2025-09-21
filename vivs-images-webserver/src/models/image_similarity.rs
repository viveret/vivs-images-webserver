use sqlx::Row;

use crate::models::image::ImageFieldMeta;


pub struct ImageSimilarity {
    pub image_comparison_key: i64,
    pub image_path_a: String,
    pub image_path_b: String,
    pub similarity_value: f64,
}

pub const IMAGE_SIMILARITY_COLUMNS_JSON: &str = r#"
[
    {"name": "image_comparison_key", "label": "Image Comparison Key", "description": "The comparison key for image A to image B", "field_type": "i64", "example": "123456", "category": "general"},
    {"name": "image_path_a", "label": "Image Path A", "description": "The file path of the left image", "field_type": "string", "example": "/images/photo.jpg", "category": "general"},
    {"name": "image_path_b", "label": "Image Path B", "description": "The file path of the right image", "field_type": "string", "example": "/images/photo.jpg", "category": "general"},
    {"name": "similarity_value", "label": "Similarity Value", "description": "The similarity of image A to image B", "field_type": "f64", "example": "0.4", "category": "general"}
]
"#;

impl ImageSimilarity {
    pub fn new(row: &sqlx::sqlite::SqliteRow) -> Self {
        let image_comparison_key: i64 = row.try_get("image_comparison_key").unwrap_or_default();
        let image_path_a: String = row.try_get("image_path_a").unwrap_or_default();
        let image_path_b: String = row.try_get("image_path_b").unwrap_or_default();
        let similarity_value: f64 = row.try_get("similarity_value").unwrap_or(0.0);

        ImageSimilarity {
            image_comparison_key,
            image_path_a,
            image_path_b,
            similarity_value,
        }
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "image_comparison_key" => Some(self.image_comparison_key.to_string()),
            "image_path_a" => Some(self.image_path_a.clone()),
            "image_path_b" => Some(self.image_path_b.clone()),
            "similarity_value" => Some(format!("{:.4}", self.similarity_value)),
            _ => None,
        }
    }
    
    pub fn get_meta() -> Vec<ImageFieldMeta> {
        serde_json::from_str::<Vec<ImageFieldMeta>>(IMAGE_SIMILARITY_COLUMNS_JSON).unwrap()
    }
}