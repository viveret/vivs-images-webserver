use std::io::ErrorKind;
use std::path::Path;
use std::io::Result;
use std::io::Error;
use std::process::Command;

use image::imageops::FilterType;
use image::GenericImageView;
use tempfile::NamedTempFile;

use crate::converters::convert_images_same_size_max::resize_to_common_dimensions;
use crate::converters::string_to_hashcode::string_hashcode_java_style;
use crate::models::image_similarity::ImageComparisonAlgorithm;
use crate::models::image_similarity::ImageSimilarity;

#[derive(Clone, Debug)]
pub struct ComputeImageSimilarityOptions {
    pub algo: ImageComparisonAlgorithm,

    pub max_dimension: Option<u32>, // Optional: maximum dimension for downscaling
    pub filter_type: Option<FilterType>, // Optional: filter type for resizing

    pub image_path_a: String,
    pub image_path_b: String,
}

impl ComputeImageSimilarityOptions {}

pub fn extract_image_similarity(options: &ComputeImageSimilarityOptions) -> Result<ImageSimilarity> {
    let (similarity_value, similarity_confidence) = match options.algo {
        ImageComparisonAlgorithm::Magick => extract_image_similarity_using_magick(options)?,
        ImageComparisonAlgorithm::CustomV1 => extract_image_similarity_using_custom_v1(options)?,
    };
    let image_comparison_key = compute_comparison_key(&options.image_path_a, &options.image_path_b);
    let image_comparison_algorithm = options.algo.clone();
    Ok(ImageSimilarity {
        image_comparison_key, image_comparison_algorithm, 
        image_path_a: options.image_path_a.clone(),
        image_path_b: options.image_path_b.clone(),
        similarity_value, similarity_confidence
    })
}

pub fn compute_comparison_key(image_path_a: &str, image_path_b: &str) -> i32 {
    string_hashcode_java_style(image_path_a).wrapping_add(string_hashcode_java_style(image_path_b))
}

fn extract_image_similarity_using_custom_v1(options: &ComputeImageSimilarityOptions) -> Result<(f32, f32)> {
    let img_a = image::open(Path::new(&options.image_path_a))
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    let img_b = image::open(Path::new(&options.image_path_b))
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    // Convert to grayscale first for faster processing
    let gray_img_a = img_a.grayscale();
    let gray_img_b = img_b.grayscale();

    // Resize images to common dimensions
    let (resized_a, resized_b) = resize_to_common_dimensions(
        &gray_img_a, 
        &gray_img_b, 
        options.max_dimension,
        options.filter_type.unwrap_or(FilterType::Lanczos3)
    );

    let (width, height) = resized_a.dimensions();
    let total_pixels = (width * height) as f64;

    // Extract pixel data as vectors for faster access
    let pixels_a: Vec<f64> = resized_a.pixels().map(|p| p.2[0] as f64).collect();
    let pixels_b: Vec<f64> = resized_b.pixels().map(|p| p.2[0] as f64).collect();

    // Calculate mean luminance for both images
    let sum_a: f64 = pixels_a.iter().sum();
    let sum_b: f64 = pixels_b.iter().sum();
    
    let mean_a = sum_a / total_pixels;
    let mean_b = sum_b / total_pixels;

    // Calculate covariance and variances
    let mut covariance = 0f64;
    let mut variance_a = 0f64;
    let mut variance_b = 0f64;

    for i in 0..pixels_a.len() {
        let diff_a = pixels_a[i] - mean_a;
        let diff_b = pixels_b[i] - mean_b;
        
        covariance += diff_a * diff_b;
        variance_a += diff_a * diff_a;
        variance_b += diff_b * diff_b;
    }

    covariance /= total_pixels;
    variance_a /= total_pixels;
    variance_b /= total_pixels;

    // Calculate similarity using Structural Similarity Index (SSIM) inspired approach
    let c1 = 6.5025; // (0.01 * 255)^2
    let c2 = 58.5225; // (0.03 * 255)^2
    
    let numerator = (2.0 * mean_a * mean_b + c1) * (2.0 * covariance + c2);
    let denominator = (mean_a * mean_a + mean_b * mean_b + c1) * (variance_a + variance_b + c2);
    
    let similarity = if denominator == 0.0 {
        1.0 // Images are identical (both likely solid color)
    } else {
        (numerator / denominator) as f32
    };

    // Calculate confidence based on variance
    let avg_variance = ((variance_a + variance_b) / 2.0) as f32;
    let confidence = (avg_variance / 65025.0).min(1.0).max(0.0);

    Ok((similarity, confidence))
}

fn extract_image_similarity_using_magick(options: &ComputeImageSimilarityOptions) -> Result<(f32, f32)> {
    // Create a temporary file
    let temp_diff_image = NamedTempFile::new().unwrap();
    let temp_path = temp_diff_image.path().to_str()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Invalid temp path"))?;

    // Run ImageMagick compare command
    let output = Command::new("magick")
        .args(["compare", "-metric", "PSNR", &options.image_path_a, &options.image_path_b, temp_path])
        .output()?;

    // println!("output.status.success(): {}", output.status.success());
    let error_msg = String::from_utf8_lossy(&output.stderr);

    // Parse the PSNR value from stderr (ImageMagick outputs metrics to stderr)
    let psnr_str = error_msg.trim().split(' ').collect::<Vec<&str>>();
    if psnr_str.len() == 2 {
        let v = psnr_str.get(0).unwrap();
        let v = v.parse::<f32>().map_err(|e| Error::new(ErrorKind::Other, format!("Failed to parse PSNR value {}: {}", v, e)))?;

        let c = psnr_str.get(1).unwrap();
        let c = c.trim_matches(['(', ')']).parse::<f32>().map_err(|e| Error::new(ErrorKind::Other, format!("Failed to parse PSNR c value {}: {}", c, e)))?;
        Ok((v, c))
    } else {
        return Err(Error::new(ErrorKind::Other, format!("ImageMagick error: {}", error_msg)));
    }
}
