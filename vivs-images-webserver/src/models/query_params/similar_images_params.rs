use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SimilarImagesParams {
    pub image_path: String,
    pub threshold: Option<f64>,
}