
/// Calculate scale factor to fit within max dimension while maintaining aspect ratio
pub fn calculate_scale_factor(width: u32, height: u32, max_dimension: u32) -> f32 {
    let max_dim = max_dimension as f32;
    let scale_width = max_dim / width as f32;
    let scale_height = max_dim / height as f32;
    
    scale_width.min(scale_height).min(1.0) // Don't upscale, only downscale
}