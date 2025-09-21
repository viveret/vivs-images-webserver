use std::time::SystemTime;

use sqlx::Row;


pub struct TopLevelMetrics {
    pub total_images: u64,
    pub categories: u64,
    pub last_updated: SystemTime,
}

impl TopLevelMetrics {
    pub fn new(results: Vec<sqlx::sqlite::SqliteRow>) -> Self {
        Self { 
            total_images: results.iter().map(|row| row.try_get::<i64, _>("total_images").unwrap_or_default()).max().unwrap_or_default() as u64,
            categories: results.iter().map(|row| row.try_get::<i64, _>("total_categories").unwrap_or_default()).max().unwrap_or_default() as u64,
            // get when DB_FILE was last updated
            last_updated: std::fs::metadata(crate::DB_FILE)
                .and_then(|meta| meta.modified())
                .unwrap_or(SystemTime::now()),
        }
    }
}