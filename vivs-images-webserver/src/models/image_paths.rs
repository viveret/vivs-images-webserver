use std::collections::HashSet;

use crate::models::image::ImageFieldMeta;



pub struct ImagePaths(pub HashSet<String>);

impl std::fmt::Display for ImagePaths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

pub const IMAGE_PATHS_COLUMNS_JSON: &str = r#"
[
    {"name": "image_path", "label": "Image Path", "description": "The file path of the image", "field_type": "string", "example": "/images/photo.jpg", "category": "general", "table_name": "image_paths"}
]
"#;

impl ImagePaths {
    pub fn get_meta() -> Vec<ImageFieldMeta> {
        serde_json::from_str::<Vec<ImageFieldMeta>>(IMAGE_PATHS_COLUMNS_JSON).unwrap()
    }
}