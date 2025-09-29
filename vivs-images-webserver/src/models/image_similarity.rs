use std::fmt::{Display};

use sqlx::Row;

use crate::models::image::ImageFieldMeta;


#[derive(Clone, Debug)]
pub enum ImageComparisonAlgorithm {
    Magick,
    CustomV1,
    CustomV2Thumbnails
}

impl TryFrom<u8> for ImageComparisonAlgorithm {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Magick),
            1 => Ok(Self::CustomV1),
            _ => Err(())
        }
    }
}

impl TryInto<u8> for &ImageComparisonAlgorithm {
    type Error = ();

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            ImageComparisonAlgorithm::Magick => Ok(0),
            ImageComparisonAlgorithm::CustomV1 => Ok(1),
            ImageComparisonAlgorithm::CustomV2Thumbnails => Ok(2),
        }
    }
}

impl Display for ImageComparisonAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: u8 = self.try_into().unwrap();
        f.write_str(&format!("{}", v))
    }
}

pub struct ImageSimilarity {
    pub image_comparison_key: i32,
    pub image_comparison_algorithm: ImageComparisonAlgorithm,
    pub image_path_a: String,
    pub image_path_b: String,
    pub similarity_value: f32,
    pub similarity_confidence: f32,
}

impl std::fmt::Display for ImageSimilarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "similarity_value: {}", self.similarity_value)
    }
}

pub const IMAGE_SIMILARITY_COLUMNS_JSON: &str = r#"
[
    {"name": "image_comparison_key", "label": "Image Comparison Key", "description": "The comparison key for image A to image B", "field_type": "i32", "example": "123456", "category": "general"},
    {"name": "image_comparison_algorithm", "label": "Image Comparison Algorithm", "description": "The comparison algorithm for image A to image B", "field_type": "u8", "example": "0", "category": "general"},
    {"name": "image_path_a", "label": "Image Path A", "description": "The file path of the left image", "field_type": "string", "example": "/images/photo.jpg", "category": "general"},
    {"name": "image_path_b", "label": "Image Path B", "description": "The file path of the right image", "field_type": "string", "example": "/images/photo.jpg", "category": "general"},
    {"name": "similarity_value", "label": "Similarity Value", "description": "The similarity of image A to image B", "field_type": "f64", "example": "0.4", "category": "general"},
    {"name": "similarity_confidence", "label": "Similarity Confidence", "description": "The reliability of the similarity value of image A to image B", "field_type": "f64", "example": "0.4", "category": "general"}
]
"#;

impl ImageSimilarity {
    pub fn new(row: &sqlx::sqlite::SqliteRow) -> Self {
        let image_comparison_key: i32 = row.try_get("image_comparison_key").unwrap_or_default();
        let image_comparison_algorithm: u8 = row.try_get("image_comparison_algorithm").unwrap_or_default();
        let image_comparison_algorithm = image_comparison_algorithm.try_into().unwrap();
        let image_path_a: String = row.try_get("image_path_a").unwrap_or_default();
        let image_path_b: String = row.try_get("image_path_b").unwrap_or_default();
        let similarity_value: f32 = row.try_get("similarity_value").unwrap_or(0.0);
        let similarity_confidence: f32 = row.try_get("similarity_confidence").unwrap_or(-1.0);

        ImageSimilarity {
            image_comparison_key,
            image_comparison_algorithm,
            image_path_a,
            image_path_b,
            similarity_value,
            similarity_confidence,
        }
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "image_comparison_key" => Some(self.image_comparison_key.to_string()),
            "image_comparison_algorithm" => Some(self.image_comparison_algorithm.to_string()),
            "image_path_a" => Some(self.image_path_a.clone()),
            "image_path_b" => Some(self.image_path_b.clone()),
            "similarity_value" => Some(format!("{:.4}", self.similarity_value)),
            "similarity_confidence" => Some(format!("{:.4}", self.similarity_confidence)),
            _ => None,
        }
    }
    
    pub fn get_meta() -> Vec<ImageFieldMeta> {
        serde_json::from_str::<Vec<ImageFieldMeta>>(IMAGE_SIMILARITY_COLUMNS_JSON).unwrap()
    }
}