use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::image_thumbnail::ImageThumbnail;

#[derive(Clone, Debug)]
pub struct ThumbnailCache {
    inner: Arc<RwLock<HashMap<String, ImageThumbnail>>>,
}

impl ThumbnailCache {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn get(&self, image_path: &str) -> Option<ImageThumbnail> {
        let cache = self.inner.read().await;
        cache.get(image_path).cloned()
    }

    pub async fn insert(&self, image_path: String, thumbnail: ImageThumbnail) {
        let mut cache = self.inner.write().await;
        cache.insert(image_path, thumbnail);
    }

    pub async fn batch_insert(&self, thumbnails: Vec<(String, ImageThumbnail)>) {
        let mut cache = self.inner.write().await;
        for (path, thumb) in thumbnails {
            cache.insert(path, thumb);
        }
    }

    // Clone is cheap - just increments Arc reference count
    pub fn clone_cache(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}