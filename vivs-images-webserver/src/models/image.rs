use serde::Deserialize;
use sqlx::Row;

use crate::models::image_aspect_ratio::ImageAspectRatio;
use crate::models::image_iptc::ImageIptc;
use crate::models::image_ocr_text::ImageOcrText;
use crate::models::image_paths::ImagePaths;
use crate::models::image_similarity::ImageSimilarity;
use crate::models::image_exif::ImageExif;
use crate::models::image_brightness::ImageBrightness;
use crate::models::image_thumbnail::ImageThumbnail;
use crate::models::image_xmp::ImageXmp;

#[derive(Debug, Clone, Deserialize)]
pub struct ImageFieldMeta {
    pub name: String,
    pub table_name: String,
    pub label: String,
    pub description: String,
    pub field_type: String,
    pub default: Option<String>,
    pub example: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub path: String,
    pub exif: Option<ImageExif>,
    pub brightness: Option<ImageBrightness>,
    pub similarity: Option<ImageSimilarity>,
    pub thumbnail: Option<ImageThumbnail>,
    pub ocr_text: Option<ImageOcrText>,
    pub aspect_ratio: Option<ImageAspectRatio>,
    pub xmp: Option<ImageXmp>,
    pub iptc: Option<ImageIptc>,
}

impl Image {
    pub fn new(row: &sqlx::sqlite::SqliteRow, tables_selected: Vec<String>) -> Self {
        let path: String = match row.try_get::<String, &str>("image_path") {
            Ok(s) => {
                if s.is_empty() {
                    panic!("image_path is empty");
                } else {
                    s
                }
            },
            Err(e) => panic!("error getting image_path: {}", e),
        };

        let exif = ImageExif::new(row);
        let brightness = ImageBrightness::new(row);
        let ocr_text = ImageOcrText::new(row);
        let aspect_ratio = ImageAspectRatio::new(row);
        let xmp = ImageXmp::new(row);
        let iptc = ImageIptc::new(row);
        let similarity = if tables_selected.contains(&"image_similarity".to_string()) {
            Some(ImageSimilarity::new(row))
        } else {
            None
        };
        let thumbnail = if tables_selected.contains(&"image_thumbnail".to_string()) {
            ImageThumbnail::new_from_row(row).ok()
        } else {
            None
        };

        Image {
            path,
            exif: Some(exif),
            brightness: Some(brightness),
            ocr_text: Some(ocr_text),
            aspect_ratio: Some(aspect_ratio),
            xmp: Some(xmp),
            iptc: Some(iptc),
            similarity,
            thumbnail,
        }
    }

    pub fn get_meta() -> Vec<ImageFieldMeta> {
        let mut x: Vec<ImageFieldMeta> = ImagePaths::get_meta();
        x.extend_from_slice(&ImageExif::get_meta()[1..]);
        x.extend_from_slice(&ImageBrightness::get_meta()[1..]);
        x.extend_from_slice(&ImageOcrText::get_meta()[1..]);
        x.extend_from_slice(&ImageAspectRatio::get_meta()[1..]);
        x.extend_from_slice(&ImageIptc::get_meta()[1..]);
        x.extend_from_slice(&ImageXmp::get_meta()[1..]);
        x
    }

    pub fn get_all_meta() -> Vec<ImageFieldMeta> {
        let mut x: Vec<ImageFieldMeta> = Self::get_meta();
        x.extend_from_slice(&ImageSimilarity::get_meta());
        x
    }

    pub fn get_meta_field(name: &str) -> Option<ImageFieldMeta> {
        Self::get_meta().iter().find(|c| c.name == name).cloned()
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
        if let Some(v) = self.ocr_text.as_ref().and_then(|s| s.get_field(field)) {
            return Some(v);
        }
        if let Some(v) = self.aspect_ratio.as_ref().and_then(|s| s.get_field(field)) {
            return Some(v);
        }
        if let Some(v) = self.iptc.as_ref().and_then(|s| s.get_field(field)) {
            return Some(v);
        }
        if let Some(v) = self.xmp.as_ref().and_then(|s| s.get_field(field)) {
            return Some(v);
        }
        None
    }
    
    pub fn assign_thumbnail(&mut self, thumb: ImageThumbnail) {
        self.thumbnail.replace(thumb);
    }
}