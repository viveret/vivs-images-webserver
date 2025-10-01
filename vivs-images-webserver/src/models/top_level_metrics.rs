use std::time::SystemTime;

use sqlx::Row;


pub struct TopLevelMetrics {
    pub total_images: u32,
    pub total_similarity: u32,
    pub total_brightness: u32,
    pub total_thumbnails: u32,
    pub total_ocr_text: u32,
    pub categories: u32,
    pub last_updated: SystemTime,
}

impl TopLevelMetrics {
    pub fn new(results: Vec<sqlx::sqlite::SqliteRow>) -> Self {
        Self { 
            total_images: results.iter().map(|row| row.try_get::<u32, _>("total_images").unwrap_or_default()).max().unwrap_or_default(),
            total_similarity: results.iter().map(|row| row.try_get::<u32, _>("total_similarity").unwrap_or_default()).max().unwrap_or_default(),
            total_brightness: results.iter().map(|row| row.try_get::<u32, _>("total_brightness").unwrap_or_default()).max().unwrap_or_default(),
            total_thumbnails: results.iter().map(|row| row.try_get::<u32, _>("total_thumbnails").unwrap_or_default()).max().unwrap_or_default(),
            total_ocr_text: results.iter().map(|row| row.try_get::<u32, _>("total_ocr_text").unwrap_or_default()).max().unwrap_or_default(),
            categories: results.iter().map(|row| row.try_get::<u32, _>("total_categories").unwrap_or_default()).max().unwrap_or_default(),
            // get when DB_FILE was last updated
            last_updated: std::fs::metadata(crate::DB_FILE)
                .and_then(|meta| meta.modified())
                .unwrap_or(SystemTime::now()),
        }
    }
}