use std::{collections::HashSet, hash::Hash};

use serde::Deserialize;
use sqlx::Row;

use crate::models::image::ImageFieldMeta;

// Struct to hold mapping of an image path to a tag
#[derive(Debug, Clone, Deserialize, Eq)]
pub struct ImageTag {
    pub image_tag_id: Option<u32>,
    pub image_path: String,
    pub tag_name: String,
}

impl std::cmp::PartialEq for ImageTag {
    fn eq(&self, other: &Self) -> bool {
        self.image_path == other.image_path && self.tag_name == other.tag_name
    }
}

impl Hash for ImageTag {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.image_path.hash(state);
        self.tag_name.hash(state);
    }
}

pub struct ImageTagSet(pub HashSet<ImageTag>);
impl ImageTagSet {
    pub fn new(tags: HashSet<ImageTag>) -> Self {
        Self(tags)
    }

    pub fn new_from_strings(image_path: String, tags: HashSet<String>) -> Self {
        Self(tags.iter().map(|tag_name| ImageTag { image_tag_id: None, image_path: image_path.clone(), tag_name: tag_name.clone() }).collect())
    }
}

impl std::fmt::Display for ImageTagSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

// Struct to hold information about a tag
#[derive(Debug, Clone, Deserialize, Eq)]
pub struct Tag {
    pub tag_name: String,
    pub tag_label: String,
    pub tag_description: String,
}

pub const IMAGE_TAG_COLUMNS_JSON: &str = r#"
[
    {"name": "image_tag_id", "label": "Image Tag ID", "description": "The id of image tag instance", "field_type": "u32", "example": "3", "category": "general", "table_name": "image_tags"},
    {"name": "image_path", "label": "Image Path", "description": "The file path of the image", "field_type": "string", "example": "/images/photo.jpg", "category": "general", "table_name": "image_tags"},
    {"name": "tag_name", "label": "Tag Name", "description": "The name of the tag associated with the image", "field_type": "string", "example": "landscape", "category": "tags", "table_name": "image_tags"}
]"#;

pub const TAG_COLUMNS_JSON: &str = r#"
[
    {"name": "tag_name", "label": "Tag Name", "description": "The unique identifier for the tag", "field_type": "string", "example": "landscape", "category": "tags", "table_name": "tags"},
    {"name": "tag_label", "label": "Tag Label", "description": "The display name for the tag", "field_type": "string", "example": "Landscape", "category": "tags", "table_name": "tags"},
    {"name": "tag_description", "label": "Tag Description", "description": "A description of what the tag represents", "field_type": "string", "example": "Images featuring natural landscapes", "category": "tags", "table_name": "tags"}
]"#;

impl ImageTag {
    pub fn new(row: &sqlx::sqlite::SqliteRow) -> Self {
        let image_tag_id: u32 = row.try_get("image_tag_id").unwrap_or_default();
        let image_path: String = row.try_get("image_path").unwrap_or_default();
        let tag_name: String = row.try_get("tag_name").unwrap_or_default();
        ImageTag {
            image_tag_id: Some(image_tag_id),
            image_path,
            tag_name,
        }
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "image_tag_id" => self.image_tag_id.map(|id| id.to_string()),
            "image_path" => Some(self.image_path.clone()),
            "tag_name" => Some(self.tag_name.clone()),
            _ => None,
        }
    }
    
    pub fn get_meta() -> Vec<ImageFieldMeta> {
        serde_json::from_str::<Vec<ImageFieldMeta>>(IMAGE_TAG_COLUMNS_JSON).unwrap()
    }
}

impl Tag {
    pub fn new(row: &sqlx::sqlite::SqliteRow) -> Self {
        let tag_name: String = row.try_get("tag_name").unwrap_or_default();
        let tag_label: String = row.try_get("tag_label").unwrap_or_default();
        let tag_description: String = row.try_get("tag_description").unwrap_or_default();
        Tag {
            tag_name,
            tag_label,
            tag_description,
        }
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "tag_name" => Some(self.tag_name.clone()),
            "tag_label" => Some(self.tag_label.clone()),
            "tag_description" => Some(self.tag_description.clone()),
            _ => None,
        }
    }
    
    pub fn get_meta() -> Vec<ImageFieldMeta> {
        serde_json::from_str::<Vec<ImageFieldMeta>>(TAG_COLUMNS_JSON).unwrap()
    }
}

impl std::cmp::PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.tag_name == other.tag_name
    }
}

impl Hash for Tag {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tag_name.hash(state);
    }
}

impl std::fmt::Display for ImageTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.image_path, self.tag_name)
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}) - {}", self.tag_name, self.tag_label, self.tag_description)
    }
}

#[derive(Debug)]
pub struct TagMetrics {
    pub tag: Tag,
    pub use_count: usize,
    pub related_tags: Vec<(Tag, usize)>,
}