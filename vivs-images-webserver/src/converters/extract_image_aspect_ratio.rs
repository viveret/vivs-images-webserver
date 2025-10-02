use std::path::Path;

use image::ImageError;
use image::GenericImageView;

use crate::models::image_aspect_ratio::ImageAspectRatio;
use crate::models::image_aspect_ratio::ImageQuality;



pub fn extract_image_aspect_ratio_model(image_path: &str) -> Result<ImageAspectRatio, ImageError> {
    let img = image::open(Path::new(image_path)).map_err(|e| e)?;
    let (width, height) = img.dimensions();
    let aspect_ratio = width as f32 / height as f32;
    let quality = if width >= 3840 || height >= 2160 {
        ImageQuality::XHD
    } else if width >= 1920 || height >= 1080 {
        ImageQuality::HD
    } else if width >= 1280 || height >= 720 {
        ImageQuality::Med
    } else {
        ImageQuality::Low
    };
    Ok(ImageAspectRatio {
        image_path: image_path.to_string(),
        width_pixels: width,
        height_pixels: height,
        aspect_ratio,
        quality
    })
}