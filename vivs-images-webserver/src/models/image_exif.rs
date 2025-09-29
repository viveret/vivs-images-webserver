use serde::Deserialize;
use sqlx::Row;

use crate::models::image::ImageFieldMeta;


// Struct to hold image EXIF data
#[derive(Debug, Clone, Deserialize)]
pub struct ImageExif {
    pub image_path: String,
    pub image_taken_at: Option<String>,
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub lens_model: Option<String>,
    pub exposure_time: Option<String>,
    pub f_number: Option<f64>,
    pub iso_speed: Option<i32>,
    pub focal_length: Option<f64>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub orientation: Option<i32>,
    pub gps_latitude: Option<f64>,
    pub gps_longitude: Option<f64>,
    pub gps_altitude: Option<f64>,
}

pub const IMAGE_EXIF_COLUMNS_JSON: &str = r#"
[
    {"name": "image_path", "label": "Image Path", "description": "The file path of the image", "field_type": "string", "example": "/images/photo.jpg", "category": "general"},
    {"name": "image_taken_at", "label": "Taken At", "description": "The date and time when the image was taken", "field_type": "datetime", "example": "2023-01-01T12:00:00Z", "category": "exif"},
    {"name": "camera_make", "label": "Camera Make", "description": "The manufacturer of the camera", "field_type": "string", "example": "Canon", "category": "exif"},
    {"name": "camera_model", "label": "Camera Model", "description": "The model of the camera", "field_type": "string", "example": "EOS 5D Mark IV", "category": "exif"},
    {"name": "lens_model", "label": "Lens Model", "description": "The model of the lens used", "field_type": "string", "example": "EF24-70mm f/2.8L II USM", "category": "exif"},
    {"name": "exposure_time", "label": "Exposure Time", "description": "The exposure time of the image in seconds", "field_type": "string", "example": "1/200 sec", "category": "exif"},
    {"name": "f_number", "label": "F-Number", 	"description":	"The F-number (aperture) of the lens when the image was taken","field_type":"float","example":"2.8","category":"exif"},
    {"name":"iso_speed","label":"ISO Speed","description":"The ISO speed rating of the camera when the image was taken","field_type":"integer","example":"100","category":"exif"},
    {"name":"focal_length","label":"Focal Length","description":"The focal length of the lens in millimeters","field_type":"float","example":"50.0","category":"exif"},
    {"name":"width","label":"Width","description":"The width of the image in pixels","field_type":"integer","example":"1920","category":"exif"},
    {"name":"height","label":"Height","description":"The height of the image in pixels","field_type":"integer","example":"1080","category":"exif"},
    {"name":"orientation","label":"Orientation","description":"The orientation of the image","field_type":"integer","example":"1","category":"exif"},
    {"name":"gps_latitude","label":"GPS Latitude","description":"The GPS latitude where the image was taken","field_type":"float","example":"37.7749","category":"exif"},
    {"name":"gps_longitude","label":"GPS Longitude","description":"The GPS longitude where the image was taken","field_type":"float","example":"-122.4194","category":"exif"},
    {"name":"gps_altitude","label":"GPS Altitude","description":"The GPS altitude where the image was taken in meters","field_type":"float","example":"15.0","category":"exif"}
]"#;

pub fn multi_try_get(row: &sqlx::sqlite::SqliteRow, fields: &[&str]) -> Option<String> {
    fields.iter().map(|&field| row.try_get(field).ok()).nth(0).flatten()
}

pub fn multi_try_get_prefixed(row: &sqlx::sqlite::SqliteRow, prefix: &str, field: &str) -> Option<String> {
    multi_try_get(row, &[&format!("{}{}", prefix, field), field])
}

impl ImageExif {
    pub fn new(row: &sqlx::sqlite::SqliteRow) -> Self {
        ImageExif {
            image_path: row.try_get("image_path").ok().unwrap_or_default(),
            image_taken_at: row.try_get("image_taken_at").ok(),
            camera_make: row.try_get("camera_make").ok(),
            camera_model: row.try_get("camera_model").ok(),
            lens_model: row.try_get("lens_model").ok(),
            exposure_time: row.try_get("exposure_time").ok(),
            f_number: row.try_get("f_number").ok().and_then(|v: String| v.parse().ok()),
            iso_speed: row.try_get("iso_speed").ok().and_then(|v: String| v.parse().ok()),
            focal_length: row.try_get("focal_length").ok().and_then(|v: String| v.parse().ok()),
            width: row.try_get("width").ok().and_then(|v: String| v.parse().ok()),
            height: row.try_get("height").ok().and_then(|v: String| v.parse().ok()),
            orientation: row.try_get("orientation").ok().and_then(|v: String| v.parse().ok()),
            gps_latitude: row.try_get("gps_latitude").ok().and_then(|v: String| v.parse().ok()),
            gps_longitude: row.try_get("gps_longitude").ok().and_then(|v: String| v.parse().ok()),
            gps_altitude: row.try_get("gps_altitude").ok().and_then(|v: String| v.parse().ok()),
        }
    }

    pub fn get_field(&self, field: &str) -> Option<String> {
        match field {
            "image_path" => Some(self.image_path.clone()),
            "image_taken_at" => self.image_taken_at.clone(),
            "camera_make" => self.camera_make.clone(),
            "camera_model" => self.camera_model.clone(),
            "lens_model" => self.lens_model.clone(),
            "exposure_time" => self.exposure_time.clone(),
            "f_number" => self.f_number.map(|v| v.to_string()),
            "iso_speed" => self.iso_speed.map(|v| v.to_string()),
            "focal_length" => self.focal_length.map(|v| v.to_string()),
            "width" => self.width.map(|v| v.to_string()),
            "height" => self.height.map(|v| v.to_string()),
            "orientation" => self.orientation.map(|v| v.to_string()),
            "gps_latitude" => self.gps_latitude.map(|v| v.to_string()),
            "gps_longitude" => self.gps_longitude.map(|v| v.to_string()),
            "gps_altitude" => self.gps_altitude.map(|v| v.to_string()),
            _ => None,
        }
    }
    
    pub fn get_meta() -> Vec<ImageFieldMeta> {
        serde_json::from_str::<Vec<ImageFieldMeta>>(IMAGE_EXIF_COLUMNS_JSON).unwrap()
    }
}

impl std::fmt::Display for ImageExif {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "image_taken_at: {:?}, ", self.image_taken_at)?;
        write!(f, "camera_make: {:?}, ", self.camera_make)?;
        write!(f, "orientation: {:?}", self.orientation)?;
        Ok(())
    }
}