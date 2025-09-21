use std::{error::Error, pin::Pin};

use exif::{In, Tag};

use crate::{converters::extract_image_taken_at::extract_image_taken_at, models::image_exif::ImageExif};


pub enum ImageToExifAlgo {
    SimpleExifRS,
}

pub struct ImageToExifOptions {
    pub algo: ImageToExifAlgo,
}

impl ImageToExifOptions {}

// Optional: Function to calculate exif of an image (equivalent to the bash image processing)
// This would require additional dependencies like image-rs
pub fn extract_image_exif(image_path: &str, options: &ImageToExifOptions) -> actix_web::Result<ImageExif> {
    let file = std::fs::File::open(image_path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("{}", e)))?;

    let camera_make = exif.get_field(Tag::Make, In::PRIMARY).map(|v| v.display_value().to_string());
    let camera_model = exif.get_field(Tag::Model, In::PRIMARY).map(|v| v.display_value().to_string());
    let lens_model = exif.get_field(Tag::LensModel, In::PRIMARY).map(|v| v.display_value().to_string());
    let exposure_time = exif.get_field(Tag::ExposureTime, In::PRIMARY).map(|v| v.display_value().to_string());
    let f_number = exif.get_field(Tag::FNumber, In::PRIMARY).map(|v| v.display_value().to_string().parse().ok()).flatten();
    let iso_speed = exif.get_field(Tag::ISOSpeed, In::PRIMARY).map(|v| v.display_value().to_string().parse().ok()).flatten();
    let focal_length = exif.get_field(Tag::FocalLength, In::PRIMARY).map(|v| v.display_value().to_string().parse().ok()).flatten();
    let width = exif.get_field(Tag::ImageWidth, In::PRIMARY).map(|v| v.display_value().to_string().parse().ok()).flatten();
    let height = exif.get_field(Tag::ImageLength, In::PRIMARY).map(|v| v.display_value().to_string().parse().ok()).flatten();
    let orientation = exif.get_field(Tag::Orientation, In::PRIMARY).map(|v| v.display_value().to_string().parse().ok()).flatten();
    let gps_latitude= exif.get_field(Tag::GPSLatitude, In::PRIMARY).map(|v| v.display_value().to_string().parse().ok()).flatten();
    let gps_longitude= exif.get_field(Tag::GPSLongitude, In::PRIMARY).map(|v| v.display_value().to_string().parse().ok()).flatten();
    let gps_altitude= exif.get_field(Tag::GPSAltitude, In::PRIMARY).map(|v| v.display_value().to_string().parse().ok()).flatten();

    let image_taken_at = extract_image_taken_at(image_path)?;
    Ok(ImageExif { image_path: image_path.to_string(), image_taken_at, camera_make, camera_model, lens_model, exposure_time, f_number, iso_speed, focal_length, width, height, orientation, gps_latitude, gps_longitude, gps_altitude })
}
