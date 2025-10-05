use std::error::Error;

use sqlx::SqlitePool;

use crate::{cache::thumbnail_cache::ThumbnailCache, database::query::query_image_thumbnail::query_thumbnail_table_at_most_width_length, models::image_thumbnail::ImageThumbnail};


#[derive(Clone, Debug)]
pub struct WebServerActionDataContext {
    pub pool: SqlitePool,
    pub thumbnail_cache: ThumbnailCache,
}

impl WebServerActionDataContext {
    pub fn new(
        pool: SqlitePool,
        thumbnail_cache: ThumbnailCache,
    ) -> Self {
        Self { pool, thumbnail_cache }
    }

    pub async fn open() -> anyhow::Result<Self> {
        // Connect to SQLite database
        let pool = SqlitePool::connect(&format!("sqlite:{}", crate::models::config::paths::DB_FILE))
            .await
            .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;

        Ok(Self::new(pool, ThumbnailCache::new()))
    }
    
    pub async fn get_thumbnail_at_most_width_length(&self, path: &str, arg: u32) -> Result<Option<ImageThumbnail>, Box<dyn Error + Send>> {
        if let Some(thumb) = self.thumbnail_cache.get(path).await {
            Ok(Some(thumb))
        } else if let Some(thumb) = query_thumbnail_table_at_most_width_length(path, arg, &self.pool).await
            .map_err(|e| anyhow::anyhow!("failed to query_thumbnail_table_at_most_width_length: {}", e))? {
            self.thumbnail_cache.insert(path.to_string(), thumb.clone()).await;
            Ok(Some(thumb))
        } else {
            Ok(None)
        }
    }
}
