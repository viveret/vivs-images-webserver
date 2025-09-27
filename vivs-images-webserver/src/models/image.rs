use serde::Deserialize;

use crate::models::image_similarity::ImageSimilarity;
use crate::models::image_exif::ImageExif;
use crate::models::image_brightness::ImageBrightness;
use crate::models::image_thumbnail::ImageThumbnail;

#[derive(Debug, Clone, Deserialize)]
pub struct ImageFieldMeta {
    pub name: String,
    pub label: String,
    pub description: String,
    pub field_type: String,
    pub example: Option<String>,
    pub category: Option<String>,
}

pub struct Image {
    pub path: String,
    pub exif: Option<ImageExif>,
    pub brightness: Option<ImageBrightness>,
    pub similarity: Option<ImageSimilarity>,
    pub thumbnail: Option<ImageThumbnail>,
}

impl Image {
    pub fn new(row: &sqlx::sqlite::SqliteRow) -> Self {
        let exif = ImageExif::new(row);
        let brightness = ImageBrightness::new(row);
        let similarity = ImageSimilarity::new(row);
        let path = exif.image_path.clone();
        
        Image {
            path,
            exif: Some(exif),
            brightness: Some(brightness),
            similarity: Some(similarity),
            thumbnail: None,
        }
    }

    pub fn get_meta() -> Vec<ImageFieldMeta> {
        let mut x = ImageExif::get_meta();
        x.extend_from_slice(&ImageBrightness::get_meta());
        x.extend_from_slice(&ImageSimilarity::get_meta());
        x
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        if field == "path" {
            return Some(self.path.clone());
        }
        if let Some(v) = self.exif.as_ref().and_then(|e| e.get_field(field)) {
            return Some(v);
        }
        if let Some(v) = self.brightness.as_ref().and_then(|b| b.get_field(field)) {
            return Some(v);
        }
        if let Some(v) = self.similarity.as_ref().and_then(|s| s.get_field(field)) {
            return Some(v);
        }
        None
    }
    
    pub fn assign_thumbnail(&mut self, thumb: ImageThumbnail) {
        self.thumbnail.replace(thumb);
    }
}