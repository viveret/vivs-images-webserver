use std::pin::Pin;
use std::path::Path;

use image::ImageError;
use image::GenericImageView;


pub enum ImageToBrightnessAlgo {
    SimpleImageRS,
    SimpleInHouse,
}

pub struct ImageToBrightnessOptions {
    pub algo: ImageToBrightnessAlgo,
}

impl ImageToBrightnessOptions {}

// Optional: Function to calculate brightness of an image (equivalent to the bash image processing)
// This would require additional dependencies like image-rs
pub fn extract_image_brightness(image_path: &str, _options: &ImageToBrightnessOptions) -> Result<f64, Pin<Box<ImageError>>> {
    let img = image::open(Path::new(image_path)).map_err(|e| Pin::new(Box::new(e)))?;
    let gray_img = img.grayscale();
    let (width, height) = gray_img.dimensions();
    let total_pixels = (width * height) as f64;

    let mut total_brightness = 0.0;
    
    for pixel in gray_img.pixels() {
        let luma = pixel.2;
        total_brightness += luma[0] as f64 / 255.0;
    }

    let brightness = total_brightness / total_pixels;
    Ok(brightness)
}
