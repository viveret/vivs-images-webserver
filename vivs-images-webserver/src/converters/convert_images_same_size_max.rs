use image::{imageops::FilterType, GenericImageView};

use crate::calc::image_aspect_ratio::calculate_scale_factor;


// Resizes two images to common dimensions for comparison
pub fn resize_to_common_dimensions(
    img_a: &image::DynamicImage,
    img_b: &image::DynamicImage,
    max_dimension: Option<u32>,
    filter_type: FilterType,
) -> (image::DynamicImage, image::DynamicImage) {
    let (width_a, height_a) = img_a.dimensions();
    let (width_b, height_b) = img_b.dimensions();

    // If images are already the same size and no downscaling needed, return as is
    if width_a == width_b && height_a == height_b && max_dimension.is_none() {
        return (img_a.clone(), img_b.clone());
    }

    // Calculate target dimensions
    let (target_width, target_height) = if let Some(max_dim) = max_dimension {
        // Calculate scale factor to fit within max dimension while maintaining aspect ratio
        let scale_a = calculate_scale_factor(width_a, height_a, max_dim);
        let scale_b = calculate_scale_factor(width_b, height_b, max_dim);
        
        // Use the smaller scale to ensure both images fit
        let scale = scale_a.min(scale_b);
        
        (
            (width_a as f32 * scale) as u32,
            (height_a as f32 * scale) as u32
        )
    } else {
        // Use the smaller dimensions to ensure both images can be compared
        (
            width_a.min(width_b),
            height_a.min(height_b)
        )
    };

    // Resize both images to target dimensions
    let resized_a = img_a.resize(target_width, target_height, filter_type);
    let resized_b = img_b.resize(target_width, target_height, filter_type);

    (resized_a, resized_b)
}
