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
    pub gps_altitude_ref: Option<String>,
    pub acceleration: Option<f64>,
    pub aperture_value: Option<f64>,
    pub artist: Option<String>,
    pub body_serial_number: Option<String>,
    pub bits_per_sample: Option<String>,
    pub brightness_value: Option<f64>,
    pub camera_elevation_angle: Option<f64>,
    pub camera_owner_name: Option<String>,
    pub cfa_pattern: Option<String>,
    pub color_space: Option<String>,
    pub components_configuration: Option<String>,
    pub composite_image: Option<String>,
    pub compressed_bits_per_pixel: Option<f64>,
    pub compression: Option<String>,
    pub contrast: Option<String>,
    pub copyright: Option<String>,
    pub custom_rendered: Option<String>,
    pub date_time: Option<String>,
    pub date_time_original: Option<String>,
    pub date_time_digitized: Option<String>,
    pub device_setting_description: Option<String>,
    pub digital_zoom_ratio: Option<f64>,
    pub exif_version: Option<String>,
    pub exposure_bias_value: Option<f64>,
    pub exposure_index: Option<f64>,
    pub exposure_mode: Option<String>,
    pub exposure_program: Option<String>,
    pub file_source: Option<String>,
    pub flash: Option<String>,
    pub flash_energy: Option<f64>,
    pub flashpix_version: Option<String>,
    pub focal_length_in_35mm_film: Option<f64>,
    pub focal_plane_resolution_unit: Option<String>,
    pub focal_plane_x_resolution: Option<f64>,
    pub focal_plane_y_resolution: Option<f64>,
    pub gain_control: Option<String>,
    pub gamma: Option<f64>,
    pub gps_area_information: Option<String>,
    pub gps_date_stamp: Option<String>,
    pub gps_dest_bearing: Option<f64>,
    pub gps_dest_bearing_ref: Option<String>,
    pub gps_dest_distance: Option<f64>,
    pub gps_dest_distance_ref: Option<String>,
    pub gps_dest_latitude: Option<f64>,
    pub gps_dest_latitude_ref: Option<String>,
    pub gps_dest_longitude: Option<f64>,
    pub gps_dest_longitude_ref: Option<String>,
    pub gps_differential: Option<String>,
    pub gps_dop: Option<f64>,
    pub gps_h_positioning_error: Option<f64>,
    pub gps_img_direction: Option<f64>,
    pub gps_img_direction_ref: Option<String>,
    pub gps_latitude_ref: Option<String>,
    pub gps_longitude_ref: Option<String>,
    pub gps_map_datum: Option<String>,
    pub gps_measure_mode: Option<String>,
    pub gps_processing_method: Option<String>,
    pub gps_satellites: Option<String>,
    pub gps_speed: Option<f64>,
    pub gps_speed_ref: Option<String>,
    pub gps_status: Option<String>,
    pub gps_time_stamp: Option<String>,
    pub gps_track: Option<f64>,
    pub gps_track_ref: Option<String>,
    pub gps_version_id: Option<String>,
    pub humidity: Option<f64>,
    pub image_description: Option<String>,
    pub image_unique_id: Option<String>,
    pub iso_speed_latitude_yyy: Option<i32>,
    pub iso_speed_latitude_zzz: Option<i32>,
    pub jpeg_interchange_format: Option<String>,
    pub jpeg_interchange_format_length: Option<String>,
    pub lens_make: Option<String>,
    pub lens_serial_number: Option<String>,
    pub lens_specification: Option<String>,
    pub light_source: Option<String>,
    pub maker_note: Option<String>,
    pub max_aperture_value: Option<f64>,
    pub metering_mode: Option<String>,
    pub oecf: Option<String>,
    pub offset_time: Option<String>,
    pub offset_time_digitized: Option<String>,
    pub offset_time_original: Option<String>,
    pub photometric_interpretation: Option<String>,
    pub pixel_x_dimension: Option<i32>,
    pub pixel_y_dimension: Option<i32>,
    pub planar_configuration: Option<String>,
    pub pressure: Option<f64>,
    pub primary_chromaticities: Option<String>,
    pub recommended_exposure_index: Option<i32>,
    pub reference_black_white: Option<String>,
    pub related_sound_file: Option<String>,
    pub resolution_unit: Option<String>,
    pub rows_per_strip: Option<i32>,
    pub samples_per_pixel: Option<i32>,
    pub saturation: Option<String>,
    pub scene_capture_type: Option<String>,
    pub scene_type: Option<String>,
    pub sensing_method: Option<String>,
    pub sensitivity_type: Option<String>,
    pub sharpness: Option<String>,
    pub shutter_speed_value: Option<f64>,
    pub software: Option<String>,
    pub source_exposure_times_of_composite_image: Option<String>,
    pub source_image_number_of_composite_image: Option<String>,
    pub spatial_frequency_response: Option<String>,
    pub spectral_sensitivity: Option<String>,
    pub standard_output_sensitivity: Option<i32>,
    pub strip_byte_counts: Option<String>,
    pub strip_offsets: Option<String>,
    pub sub_sec_time: Option<String>,
    pub sub_sec_time_digitized: Option<String>,
    pub sub_sec_time_original: Option<String>,
    pub subject_area: Option<String>,
    pub subject_distance: Option<f64>,
    pub subject_distance_range: Option<String>,
    pub subject_location: Option<String>,
    pub temperature: Option<f64>,
    pub tile_byte_counts: Option<String>,
    pub tile_offsets: Option<String>,
    pub transfer_function: Option<String>,
    pub user_comment: Option<String>,
    pub water_depth: Option<f64>,
    pub white_balance: Option<String>,
    pub white_point: Option<String>,
    pub x_resolution: Option<f64>,
    pub ycbcr_coefficients: Option<String>,
    pub ycbcr_positioning: Option<String>,
    pub ycbcr_sub_sampling: Option<String>,
    pub y_resolution: Option<f64>,
}

pub const IMAGE_EXIF_COLUMNS_JSON: &str = r#"
[
    {"name": "image_path", "label": "Image Path", "description": "The file path of the image", "field_type": "string", "example": "/images/photo.jpg", "category": "general", "table_name": "image_exif"},
    {"name": "image_taken_at", "label": "Taken At", "description": "The date and time when the image was taken", "field_type": "datetime", "example": "2023-01-01T12:00:00Z", "category": "exif", "table_name": "image_exif"},
    {"name": "camera_make", "label": "Camera Make", "description": "The manufacturer of the camera", "field_type": "string", "example": "Canon", "category": "exif", "table_name": "image_exif"},
    {"name": "camera_model", "label": "Camera Model", "description": "The model of the camera", "field_type": "string", "example": "EOS 5D Mark IV", "category": "exif", "table_name": "image_exif"},
    {"name": "lens_model", "label": "Lens Model", "description": "The model of the lens used", "field_type": "string", "example": "EF24-70mm f/2.8L II USM", "category": "exif", "table_name": "image_exif"},
    {"name": "exposure_time", "label": "Exposure Time", "description": "The exposure time of the image in seconds", "field_type": "string", "example": "1/200 sec", "category": "exif", "table_name": "image_exif"},
    {"name": "f_number", "label": "F-Number", "description": "The F-number (aperture) of the lens when the image was taken", "field_type": "float", "example": "2.8", "category": "exif", "table_name": "image_exif"},
    {"name": "iso_speed", "label": "ISO Speed", "description": "The ISO speed rating of the camera when the image was taken", "field_type": "integer", "example": "100", "category": "exif", "table_name": "image_exif"},
    {"name": "focal_length", "label": "Focal Length", "description": "The focal length of the lens in millimeters", "field_type": "float", "example": "50.0", "category": "exif", "table_name": "image_exif"},
    {"name": "width", "label": "Width", "description": "The width of the image in pixels", "field_type": "integer", "example": "1920", "category": "exif", "table_name": "image_exif"},
    {"name": "height", "label": "Height", "description": "The height of the image in pixels", "field_type": "integer", "example": "1080", "category": "exif", "table_name": "image_exif"},
    {"name": "orientation", "label": "Orientation", "description": "The orientation of the image", "field_type": "integer", "example": "1", "category": "exif", "table_name": "image_exif"},
    {"name": "gps_latitude", "label": "GPS Latitude", "description": "The GPS latitude where the image was taken", "field_type": "float", "example": "37.7749", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_longitude", "label": "GPS Longitude", "description": "The GPS longitude where the image was taken", "field_type": "float", "example": "-122.4194", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_altitude", "label": "GPS Altitude", "description": "The GPS altitude where the image was taken in meters", "field_type": "float", "example": "15.0", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_altitude_ref", "label": "GPS Altitude Ref", "description": "GPS altitude reference (0=above sea level, 1=below sea level)", "field_type": "string", "example": "0", "category": "gps", "table_name": "image_exif"},
    {"name": "acceleration", "label": "Acceleration", "description": "Acceleration measurement in mGal", "field_type": "float", "example": "980.665", "category": "exif", "table_name": "image_exif"},
    {"name": "aperture_value", "label": "Aperture Value", "description": "The aperture value as an APEX value", "field_type": "float", "example": "2.8", "category": "exif", "table_name": "image_exif"},
    {"name": "artist", "label": "Artist", "description": "Person who created the image", "field_type": "string", "example": "John Doe", "category": "exif", "table_name": "image_exif"},
    {"name": "body_serial_number", "label": "Body Serial Number", "description": "Camera body serial number", "field_type": "string", "example": "123456789", "category": "exif", "table_name": "image_exif"},
    {"name": "bits_per_sample", "label": "Bits Per Sample", "description": "Number of bits per component", "field_type": "string", "example": "8 8 8", "category": "exif", "table_name": "image_exif"},
    {"name": "brightness_value", "label": "Brightness Value", "description": "Brightness value in EV", "field_type": "float", "example": "8.5", "category": "exif", "table_name": "image_exif"},
    {"name": "camera_elevation_angle", "label": "Camera Elevation Angle", "description": "Camera elevation angle in degrees", "field_type": "float", "example": "45.0", "category": "exif", "table_name": "image_exif"},
    {"name": "camera_owner_name", "label": "Camera Owner Name", "description": "Name of the camera owner", "field_type": "string", "example": "Jane Smith", "category": "exif", "table_name": "image_exif"},
    {"name": "cfa_pattern", "label": "CFA Pattern", "description": "Color filter array pattern", "field_type": "string", "example": "[Red,Green][Green,Blue]", "category": "exif", "table_name": "image_exif"},
    {"name": "color_space", "label": "Color Space", "description": "Color space information", "field_type": "string", "example": "sRGB", "category": "exif", "table_name": "image_exif"},
    {"name": "components_configuration", "label": "Components Configuration", "description": "Meaning of each component", "field_type": "string", "example": "YCbCr", "category": "exif", "table_name": "image_exif"},
    {"name": "composite_image", "label": "Composite Image", "description": "Composite image information", "field_type": "string", "example": "0", "category": "exif", "table_name": "image_exif"},
    {"name": "compressed_bits_per_pixel", "label": "Compressed Bits Per Pixel", "description": "Image compression mode", "field_type": "float", "example": "4.0", "category": "exif", "table_name": "image_exif"},
    {"name": "compression", "label": "Compression", "description": "Compression scheme", "field_type": "string", "example": "JPEG", "category": "exif", "table_name": "image_exif"},
    {"name": "contrast", "label": "Contrast", "description": "Contrast setting", "field_type": "string", "example": "Normal", "category": "exif", "table_name": "image_exif"},
    {"name": "copyright", "label": "Copyright", "description": "Copyright holder", "field_type": "string", "example": "Copyright 2023", "category": "exif", "table_name": "image_exif"},
    {"name": "custom_rendered", "label": "Custom Rendered", "description": "Custom image processing", "field_type": "string", "example": "Normal", "category": "exif", "table_name": "image_exif"},
    {"name": "date_time", "label": "Date Time", "description": "File change date and time", "field_type": "datetime", "example": "2023-01-01T12:00:00Z", "category": "exif", "table_name": "image_exif"},
    {"name": "date_time_original", "label": "Date Time Original", "description": "Date and time of original data generation", "field_type": "datetime", "example": "2023-01-01T12:00:00Z", "category": "exif", "table_name": "image_exif"},
    {"name": "date_time_digitized", "label": "Date Time Digitized", "description": "Date and time of digital data generation", "field_type": "datetime", "example": "2023-01-01T12:00:00Z", "category": "exif", "table_name": "image_exif"},
    {"name": "device_setting_description", "label": "Device Setting Description", "description": "Device settings description", "field_type": "string", "example": "Portrait mode", "category": "exif", "table_name": "image_exif"},
    {"name": "digital_zoom_ratio", "label": "Digital Zoom Ratio", "description": "Digital zoom ratio", "field_type": "float", "example": "1.0", "category": "exif", "table_name": "image_exif"},
    {"name": "exif_version", "label": "Exif Version", "description": "Exif version", "field_type": "string", "example": "0230", "category": "exif", "table_name": "image_exif"},
    {"name": "exposure_bias_value", "label": "Exposure Bias Value", "description": "Exposure bias in EV", "field_type": "float", "example": "0.0", "category": "exif", "table_name": "image_exif"},
    {"name": "exposure_index", "label": "Exposure Index", "description": "Exposure index", "field_type": "float", "example": "100", "category": "exif", "table_name": "image_exif"},
    {"name": "exposure_mode", "label": "Exposure Mode", "description": "Exposure mode", "field_type": "string", "example": "Auto", "category": "exif", "table_name": "image_exif"},
    {"name": "exposure_program", "label": "Exposure Program", "description": "Exposure program", "field_type": "string", "example": "Aperture priority", "category": "exif", "table_name": "image_exif"},
    {"name": "file_source", "label": "File Source", "description": "File source", "field_type": "string", "example": "Digital Camera", "category": "exif", "table_name": "image_exif"},
    {"name": "flash", "label": "Flash", "description": "Flash information", "field_type": "string", "example": "Fired, auto", "category": "exif", "table_name": "image_exif"},
    {"name": "flash_energy", "label": "Flash Energy", "description": "Flash energy in BCPS", "field_type": "float", "example": "100.0", "category": "exif", "table_name": "image_exif"},
    {"name": "flashpix_version", "label": "Flashpix Version", "description": "Supported Flashpix version", "field_type": "string", "example": "0100", "category": "exif", "table_name": "image_exif"},
    {"name": "focal_length_in_35mm_film", "label": "Focal Length in 35mm Film", "description": "Focal length in 35 mm film equivalent", "field_type": "float", "example": "75.0", "category": "exif", "table_name": "image_exif"},
    {"name": "focal_plane_resolution_unit", "label": "Focal Plane Resolution Unit", "description": "Focal plane resolution unit", "field_type": "string", "example": "inches", "category": "exif", "table_name": "image_exif"},
    {"name": "focal_plane_x_resolution", "label": "Focal Plane X Resolution", "description": "Focal plane X resolution", "field_type": "float", "example": "300.0", "category": "exif", "table_name": "image_exif"},
    {"name": "focal_plane_y_resolution", "label": "Focal Plane Y Resolution", "description": "Focal plane Y resolution", "field_type": "float", "example": "300.0", "category": "exif", "table_name": "image_exif"},
    {"name": "gain_control", "label": "Gain Control", "description": "Gain control", "field_type": "string", "example": "Low gain up", "category": "exif", "table_name": "image_exif"},
    {"name": "gamma", "label": "Gamma", "description": "Gamma value", "field_type": "float", "example": "2.2", "category": "exif", "table_name": "image_exif"},
    {"name": "gps_area_information", "label": "GPS Area Information", "description": "Name of GPS area", "field_type": "string", "example": "San Francisco", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_date_stamp", "label": "GPS Date Stamp", "description": "GPS date", "field_type": "string", "example": "2023:01:01", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_dest_bearing", "label": "GPS Dest Bearing", "description": "Bearing of destination in degrees", "field_type": "float", "example": "45.0", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_dest_bearing_ref", "label": "GPS Dest Bearing Ref", "description": "Reference for bearing of destination", "field_type": "string", "example": "T", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_dest_distance", "label": "GPS Dest Distance", "description": "Distance to destination", "field_type": "float", "example": "10.5", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_dest_distance_ref", "label": "GPS Dest Distance Ref", "description": "Reference for distance to destination", "field_type": "string", "example": "K", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_dest_latitude", "label": "GPS Dest Latitude", "description": "Latitude of destination", "field_type": "float", "example": "37.7749", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_dest_latitude_ref", "label": "GPS Dest Latitude Ref", "description": "Reference for latitude of destination", "field_type": "string", "example": "N", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_dest_longitude", "label": "GPS Dest Longitude", "description": "Longitude of destination", "field_type": "float", "example": "-122.4194", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_dest_longitude_ref", "label": "GPS Dest Longitude Ref", "description": "Reference for longitude of destination", "field_type": "string", "example": "W", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_differential", "label": "GPS Differential", "description": "GPS differential correction", "field_type": "string", "example": "0", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_dop", "label": "GPS DOP", "description": "Measurement precision", "field_type": "float", "example": "2.5", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_h_positioning_error", "label": "GPS H Positioning Error", "description": "Horizontal positioning error in meters", "field_type": "float", "example": "5.0", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_img_direction", "label": "GPS Img Direction", "description": "Direction of image in degrees", "field_type": "float", "example": "180.0", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_img_direction_ref", "label": "GPS Img Direction Ref", "description": "Reference for direction of image", "field_type": "string", "example": "T", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_latitude_ref", "label": "GPS Latitude Ref", "description": "North or south latitude", "field_type": "string", "example": "N", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_longitude_ref", "label": "GPS Longitude Ref", "description": "East or west longitude", "field_type": "string", "example": "W", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_map_datum", "label": "GPS Map Datum", "description": "Geodetic survey data used", "field_type": "string", "example": "WGS-84", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_measure_mode", "label": "GPS Measure Mode", "description": "GPS measurement mode", "field_type": "string", "example": "3", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_processing_method", "label": "GPS Processing Method", "description": "Name of GPS processing method", "field_type": "string", "example": "GPS", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_satellites", "label": "GPS Satellites", "description": "GPS satellites used for measurement", "field_type": "string", "example": "G01,G03,G05", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_speed", "label": "GPS Speed", "description": "Speed of GPS receiver", "field_type": "float", "example": "5.5", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_speed_ref", "label": "GPS Speed Ref", "description": "Speed unit", "field_type": "string", "example": "K", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_status", "label": "GPS Status", "description": "GPS receiver status", "field_type": "string", "example": "A", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_time_stamp", "label": "GPS Time Stamp", "description": "GPS time (atomic clock)", "field_type": "string", "example": "12:00:00", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_track", "label": "GPS Track", "description": "Direction of movement in degrees", "field_type": "float", "example": "90.0", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_track_ref", "label": "GPS Track Ref", "description": "Reference for direction of movement", "field_type": "string", "example": "T", "category": "gps", "table_name": "image_exif"},
    {"name": "gps_version_id", "label": "GPS Version ID", "description": "GPS tag version", "field_type": "string", "example": "2.3.0.0", "category": "gps", "table_name": "image_exif"},
    {"name": "humidity", "label": "Humidity", "description": "Humidity percentage", "field_type": "float", "example": "65.5", "category": "exif", "table_name": "image_exif"},
    {"name": "image_description", "label": "Image Description", "description": "Image title", "field_type": "string", "example": "Vacation photo", "category": "exif", "table_name": "image_exif"},
    {"name": "image_unique_id", "label": "Image Unique ID", "description": "Unique image ID", "field_type": "string", "example": "abc123def456", "category": "exif", "table_name": "image_exif"},
    {"name": "iso_speed_latitude_yyy", "label": "ISO Speed Latitude YYY", "description": "ISO speed latitude yyy", "field_type": "integer", "example": "100", "category": "exif", "table_name": "image_exif"},
    {"name": "iso_speed_latitude_zzz", "label": "ISO Speed Latitude ZZZ", "description": "ISO speed latitude zzz", "field_type": "integer", "example": "100", "category": "exif", "table_name": "image_exif"},
    {"name": "jpeg_interchange_format", "label": "JPEG Interchange Format", "description": "Offset to JPEG SOI", "field_type": "string", "example": "216", "category": "exif", "table_name": "image_exif"},
    {"name": "jpeg_interchange_format_length", "label": "JPEG Interchange Format Length", "description": "Bytes of JPEG data", "field_type": "string", "example": "1024", "category": "exif", "table_name": "image_exif"},
    {"name": "lens_make", "label": "Lens Make", "description": "Lens manufacturer", "field_type": "string", "example": "Canon", "category": "exif", "table_name": "image_exif"},
    {"name": "lens_serial_number", "label": "Lens Serial Number", "description": "Lens serial number", "field_type": "string", "example": "987654321", "category": "exif", "table_name": "image_exif"},
    {"name": "lens_specification", "label": "Lens Specification", "description": "Lens specification", "field_type": "string", "example": "24-70mm f/2.8", "category": "exif", "table_name": "image_exif"},
    {"name": "light_source", "label": "Light Source", "description": "Light source", "field_type": "string", "example": "Daylight", "category": "exif", "table_name": "image_exif"},
    {"name": "maker_note", "label": "Maker Note", "description": "Manufacturer notes", "field_type": "string", "example": "Custom settings", "category": "exif", "table_name": "image_exif"},
    {"name": "max_aperture_value", "label": "Max Aperture Value", "description": "Maximum lens aperture in EV", "field_type": "float", "example": "1.0", "category": "exif", "table_name": "image_exif"},
    {"name": "metering_mode", "label": "Metering Mode", "description": "Metering mode", "field_type": "string", "example": "Multi-segment", "category": "exif", "table_name": "image_exif"},
    {"name": "oecf", "label": "OECF", "description": "Optoelectric conversion factor", "field_type": "string", "example": "OECF data", "category": "exif", "table_name": "image_exif"},
    {"name": "offset_time", "label": "Offset Time", "description": "Offset data of DateTime", "field_type": "string", "example": "+01:00", "category": "exif", "table_name": "image_exif"},
    {"name": "offset_time_digitized", "label": "Offset Time Digitized", "description": "Offset data of DateTimeDigitized", "field_type": "string", "example": "+01:00", "category": "exif", "table_name": "image_exif"},
    {"name": "offset_time_original", "label": "Offset Time Original", "description": "Offset data of DateTimeOriginal", "field_type": "string", "example": "+01:00", "category": "exif", "table_name": "image_exif"},
    {"name": "photometric_interpretation", "label": "Photometric Interpretation", "description": "Pixel composition", "field_type": "string", "example": "RGB", "category": "exif", "table_name": "image_exif"},
    {"name": "pixel_x_dimension", "label": "Pixel X Dimension", "description": "Valid image width in pixels", "field_type": "integer", "example": "1920", "category": "exif", "table_name": "image_exif"},
    {"name": "pixel_y_dimension", "label": "Pixel Y Dimension", "description": "Valid image height in pixels", "field_type": "integer", "example": "1080", "category": "exif", "table_name": "image_exif"},
    {"name": "planar_configuration", "label": "Planar Configuration", "description": "Image data arrangement", "field_type": "string", "example": "Chunky", "category": "exif", "table_name": "image_exif"},
    {"name": "pressure", "label": "Pressure", "description": "Pressure in hPa", "field_type": "float", "example": "1013.25", "category": "exif", "table_name": "image_exif"},
    {"name": "primary_chromaticities", "label": "Primary Chromaticities", "description": "Chromaticities of primaries", "field_type": "string", "example": "0.64,0.33,0.21,0.71,0.15,0.06", "category": "exif", "table_name": "image_exif"},
    {"name": "recommended_exposure_index", "label": "Recommended Exposure Index", "description": "Recommended exposure index", "field_type": "integer", "example": "100", "category": "exif", "table_name": "image_exif"},
    {"name": "reference_black_white", "label": "Reference Black White", "description": "Pair of black and white reference values", "field_type": "string", "example": "0,255", "category": "exif", "table_name": "image_exif"},
    {"name": "related_sound_file", "label": "Related Sound File", "description": "Related audio file", "field_type": "string", "example": "sound.wav", "category": "exif", "table_name": "image_exif"},
    {"name": "resolution_unit", "label": "Resolution Unit", "description": "Unit of X and Y resolution", "field_type": "string", "example": "inches", "category": "exif", "table_name": "image_exif"},
    {"name": "rows_per_strip", "label": "Rows Per Strip", "description": "Number of rows per strip", "field_type": "integer", "example": "64", "category": "exif", "table_name": "image_exif"},
    {"name": "samples_per_pixel", "label": "Samples Per Pixel", "description": "Number of components", "field_type": "integer", "example": "3", "category": "exif", "table_name": "image_exif"},
    {"name": "saturation", "label": "Saturation", "description": "Saturation setting", "field_type": "string", "example": "Normal", "category": "exif", "table_name": "image_exif"},
    {"name": "scene_capture_type", "label": "Scene Capture Type", "description": "Scene capture type", "field_type": "string", "example": "Standard", "category": "exif", "table_name": "image_exif"},
    {"name": "scene_type", "label": "Scene Type", "description": "Scene type", "field_type": "string", "example": "1", "category": "exif", "table_name": "image_exif"},
    {"name": "sensing_method", "label": "Sensing Method", "description": "Sensing method", "field_type": "string", "example": "One-chip color area", "category": "exif", "table_name": "image_exif"},
    {"name": "sensitivity_type", "label": "Sensitivity Type", "description": "Sensitivity type", "field_type": "string", "example": "SOS", "category": "exif", "table_name": "image_exif"},
    {"name": "sharpness", "label": "Sharpness", "description": "Sharpness setting", "field_type": "string", "example": "Normal", "category": "exif", "table_name": "image_exif"},
    {"name": "shutter_speed_value", "label": "Shutter Speed Value", "description": "Shutter speed in EV", "field_type": "float", "example": "8.0", "category": "exif", "table_name": "image_exif"},
    {"name": "software", "label": "Software", "description": "Software used", "field_type": "string", "example": "Adobe Photoshop", "category": "exif", "table_name": "image_exif"},
    {"name": "source_exposure_times_of_composite_image", "label": "Source Exposure Times Of Composite Image", "description": "Source exposure times of composite image", "field_type": "string", "example": "1/100,1/200", "category": "exif", "table_name": "image_exif"},
    {"name": "source_image_number_of_composite_image", "label": "Source Image Number Of Composite Image", "description": "Source image number of composite image", "field_type": "string", "example": "2", "category": "exif", "table_name": "image_exif"},
    {"name": "spatial_frequency_response", "label": "Spatial Frequency Response", "description": "Spatial frequency response", "field_type": "string", "example": "SFR data", "category": "exif", "table_name": "image_exif"},
    {"name": "spectral_sensitivity", "label": "Spectral Sensitivity", "description": "Spectral sensitivity", "field_type": "string", "example": "Unknown", "category": "exif", "table_name": "image_exif"},
    {"name": "standard_output_sensitivity", "label": "Standard Output Sensitivity", "description": "Standard output sensitivity", "field_type": "integer", "example": "100", "category": "exif", "table_name": "image_exif"},
    {"name": "strip_byte_counts", "label": "Strip Byte Counts", "description": "Bytes per compressed strip", "field_type": "string", "example": "8192", "category": "exif", "table_name": "image_exif"},
    {"name": "strip_offsets", "label": "Strip Offsets", "description": "Image data location", "field_type": "string", "example": "2048", "category": "exif", "table_name": "image_exif"},
    {"name": "sub_sec_time", "label": "Sub Sec Time", "description": "DateTime subseconds", "field_type": "string", "example": "123", "category": "exif", "table_name": "image_exif"},
    {"name": "sub_sec_time_digitized", "label": "Sub Sec Time Digitized", "description": "DateTimeDigitized subseconds", "field_type": "string", "example": "123", "category": "exif", "table_name": "image_exif"},
    {"name": "sub_sec_time_original", "label": "Sub Sec Time Original", "description": "DateTimeOriginal subseconds", "field_type": "string", "example": "123", "category": "exif", "table_name": "image_exif"},
    {"name": "subject_area", "label": "Subject Area", "description": "Subject area", "field_type": "string", "example": "100,100,50,50", "category": "exif", "table_name": "image_exif"},
    {"name": "subject_distance", "label": "Subject Distance", "description": "Subject distance in meters", "field_type": "float", "example": "5.0", "category": "exif", "table_name": "image_exif"},
    {"name": "subject_distance_range", "label": "Subject Distance Range", "description": "Subject distance range", "field_type": "string", "example": "Close", "category": "exif", "table_name": "image_exif"},
    {"name": "subject_location", "label": "Subject Location", "description": "Subject location", "field_type": "string", "example": "100,100", "category": "exif", "table_name": "image_exif"},
    {"name": "temperature", "label": "Temperature", "description": "Temperature in degrees Celsius", "field_type": "float", "example": "25.0", "category": "exif", "table_name": "image_exif"},
    {"name": "tile_byte_counts", "label": "Tile Byte Counts", "description": "Bytes per compressed tile", "field_type": "string", "example": "4096", "category": "exif", "table_name": "image_exif"},
    {"name": "tile_offsets", "label": "Tile Offsets", "description": "Tiled image data location", "field_type": "string", "example": "1024", "category": "exif", "table_name": "image_exif"},
    {"name": "transfer_function", "label": "Transfer Function", "description": "Transfer function", "field_type": "string", "example": "Linear", "category": "exif", "table_name": "image_exif"},
    {"name": "user_comment", "label": "User Comment", "description": "User comments", "field_type": "string", "example": "Beautiful sunset", "category": "exif", "table_name": "image_exif"},
    {"name": "water_depth", "label": "Water Depth", "description": "Water depth in meters", "field_type": "float", "example": "10.5", "category": "exif", "table_name": "image_exif"},
    {"name": "white_balance", "label": "White Balance", "description": "White balance", "field_type": "string", "example": "Auto", "category": "exif", "table_name": "image_exif"},
    {"name": "white_point", "label": "White Point", "description": "White point chromaticity", "field_type": "string", "example": "0.3127,0.3290", "category": "exif", "table_name": "image_exif"},
    {"name": "x_resolution", "label": "X Resolution", "description": "Image resolution in width direction", "field_type": "float", "example": "300.0", "category": "exif", "table_name": "image_exif"},
    {"name": "ycbcr_coefficients", "label": "YCbCr Coefficients", "description": "Color space transformation matrix coefficients", "field_type": "string", "example": "0.299,0.587,0.114", "category": "exif", "table_name": "image_exif"},
    {"name": "ycbcr_positioning", "label": "YCbCr Positioning", "description": "Y and C positioning", "field_type": "string", "example": "Centered", "category": "exif", "table_name": "image_exif"},
    {"name": "ycbcr_sub_sampling", "label": "YCbCr Sub Sampling", "description": "Subsampling ratio of Y to C", "field_type": "string", "example": "2,1", "category": "exif", "table_name": "image_exif"},
    {"name": "y_resolution", "label": "Y Resolution", "description": "Image resolution in height direction", "field_type": "float", "example": "300.0", "category": "exif", "table_name": "image_exif"}
]
"#;

pub fn multi_try_get(row: &sqlx::sqlite::SqliteRow, fields: &[&str]) -> Option<String> {
    fields.iter().find_map(|&field| row.try_get(field).ok())
}

pub fn multi_try_get_prefixed(row: &sqlx::sqlite::SqliteRow, prefix: &str, field: &str) -> Option<String> {
    multi_try_get(row, &[&format!("{}{}", prefix, field), field])
}

// Helper function to parse numeric values safely
fn parse_float(value: &str) -> Option<f64> {
    value.parse().ok()
}

fn parse_int(value: &str) -> Option<i32> {
    value.parse().ok()
}

impl ImageExif {
    pub fn new(row: &sqlx::sqlite::SqliteRow) -> Self {
        // Helper macro to simplify field extraction
        macro_rules! extract_field {
            ($field:ident, $type:ty) => {
                row.try_get(stringify!($field)).ok()
            };
            ($field:ident, parse $type:ty) => {
                row.try_get::<Option<String>, _>(stringify!($field))
                    .ok()
                    .flatten()
                    .and_then(|v| if v.is_empty() { None } else { v.parse().ok() })
            };
        }

        ImageExif {
            image_path: extract_field!(image_path, String).unwrap_or_default(),
            image_taken_at: extract_field!(image_taken_at, String),
            camera_make: extract_field!(camera_make, String),
            camera_model: extract_field!(camera_model, String),
            lens_model: extract_field!(lens_model, String),
            exposure_time: extract_field!(exposure_time, String),
            f_number: extract_field!(f_number, parse f64),
            iso_speed: extract_field!(iso_speed, parse i32),
            focal_length: extract_field!(focal_length, parse f64),
            width: extract_field!(width, parse i32),
            height: extract_field!(height, parse i32),
            orientation: extract_field!(orientation, parse i32),
            gps_latitude: extract_field!(gps_latitude, parse f64),
            gps_longitude: extract_field!(gps_longitude, parse f64),
            gps_altitude: extract_field!(gps_altitude, parse f64),
            gps_altitude_ref: extract_field!(gps_altitude_ref, String),
            acceleration: extract_field!(acceleration, parse f64),
            aperture_value: extract_field!(aperture_value, parse f64),
            artist: extract_field!(artist, String),
            body_serial_number: extract_field!(body_serial_number, String),
            bits_per_sample: extract_field!(bits_per_sample, String),
            brightness_value: extract_field!(brightness_value, parse f64),
            camera_elevation_angle: extract_field!(camera_elevation_angle, parse f64),
            camera_owner_name: extract_field!(camera_owner_name, String),
            cfa_pattern: extract_field!(cfa_pattern, String),
            color_space: extract_field!(color_space, String),
            components_configuration: extract_field!(components_configuration, String),
            composite_image: extract_field!(composite_image, String),
            compressed_bits_per_pixel: extract_field!(compressed_bits_per_pixel, parse f64),
            compression: extract_field!(compression, String),
            contrast: extract_field!(contrast, String),
            copyright: extract_field!(copyright, String),
            custom_rendered: extract_field!(custom_rendered, String),
            date_time: extract_field!(date_time, String),
            date_time_original: extract_field!(date_time_original, String),
            date_time_digitized: extract_field!(date_time_digitized, String),
            device_setting_description: extract_field!(device_setting_description, String),
            digital_zoom_ratio: extract_field!(digital_zoom_ratio, parse f64),
            exif_version: extract_field!(exif_version, String),
            exposure_bias_value: extract_field!(exposure_bias_value, parse f64),
            exposure_index: extract_field!(exposure_index, parse f64),
            exposure_mode: extract_field!(exposure_mode, String),
            exposure_program: extract_field!(exposure_program, String),
            file_source: extract_field!(file_source, String),
            flash: extract_field!(flash, String),
            flash_energy: extract_field!(flash_energy, parse f64),
            flashpix_version: extract_field!(flashpix_version, String),
            focal_length_in_35mm_film: extract_field!(focal_length_in_35mm_film, parse f64),
            focal_plane_resolution_unit: extract_field!(focal_plane_resolution_unit, String),
            focal_plane_x_resolution: extract_field!(focal_plane_x_resolution, parse f64),
            focal_plane_y_resolution: extract_field!(focal_plane_y_resolution, parse f64),
            gain_control: extract_field!(gain_control, String),
            gamma: extract_field!(gamma, parse f64),
            gps_area_information: extract_field!(gps_area_information, String),
            gps_date_stamp: extract_field!(gps_date_stamp, String),
            gps_dest_bearing: extract_field!(gps_dest_bearing, parse f64),
            gps_dest_bearing_ref: extract_field!(gps_dest_bearing_ref, String),
            gps_dest_distance: extract_field!(gps_dest_distance, parse f64),
            gps_dest_distance_ref: extract_field!(gps_dest_distance_ref, String),
            gps_dest_latitude: extract_field!(gps_dest_latitude, parse f64),
            gps_dest_latitude_ref: extract_field!(gps_dest_latitude_ref, String),
            gps_dest_longitude: extract_field!(gps_dest_longitude, parse f64),
            gps_dest_longitude_ref: extract_field!(gps_dest_longitude_ref, String),
            gps_differential: extract_field!(gps_differential, String),
            gps_dop: extract_field!(gps_dop, parse f64),
            gps_h_positioning_error: extract_field!(gps_h_positioning_error, parse f64),
            gps_img_direction: extract_field!(gps_img_direction, parse f64),
            gps_img_direction_ref: extract_field!(gps_img_direction_ref, String),
            gps_latitude_ref: extract_field!(gps_latitude_ref, String),
            gps_longitude_ref: extract_field!(gps_longitude_ref, String),
            gps_map_datum: extract_field!(gps_map_datum, String),
            gps_measure_mode: extract_field!(gps_measure_mode, String),
            gps_processing_method: extract_field!(gps_processing_method, String),
            gps_satellites: extract_field!(gps_satellites, String),
            gps_speed: extract_field!(gps_speed, parse f64),
            gps_speed_ref: extract_field!(gps_speed_ref, String),
            gps_status: extract_field!(gps_status, String),
            gps_time_stamp: extract_field!(gps_time_stamp, String),
            gps_track: extract_field!(gps_track, parse f64),
            gps_track_ref: extract_field!(gps_track_ref, String),
            gps_version_id: extract_field!(gps_version_id, String),
            humidity: extract_field!(humidity, parse f64),
            image_description: extract_field!(image_description, String),
            image_unique_id: extract_field!(image_unique_id, String),
            iso_speed_latitude_yyy: extract_field!(iso_speed_latitude_yyy, parse i32),
            iso_speed_latitude_zzz: extract_field!(iso_speed_latitude_zzz, parse i32),
            jpeg_interchange_format: extract_field!(jpeg_interchange_format, String),
            jpeg_interchange_format_length: extract_field!(jpeg_interchange_format_length, String),
            lens_make: extract_field!(lens_make, String),
            lens_serial_number: extract_field!(lens_serial_number, String),
            lens_specification: extract_field!(lens_specification, String),
            light_source: extract_field!(light_source, String),
            maker_note: extract_field!(maker_note, String),
            max_aperture_value: extract_field!(max_aperture_value, parse f64),
            metering_mode: extract_field!(metering_mode, String),
            oecf: extract_field!(oecf, String),
            offset_time: extract_field!(offset_time, String),
            offset_time_digitized: extract_field!(offset_time_digitized, String),
            offset_time_original: extract_field!(offset_time_original, String),
            photometric_interpretation: extract_field!(photometric_interpretation, String),
            pixel_x_dimension: extract_field!(pixel_x_dimension, parse i32),
            pixel_y_dimension: extract_field!(pixel_y_dimension, parse i32),
            planar_configuration: extract_field!(planar_configuration, String),
            pressure: extract_field!(pressure, parse f64),
            primary_chromaticities: extract_field!(primary_chromaticities, String),
            recommended_exposure_index: extract_field!(recommended_exposure_index, parse i32),
            reference_black_white: extract_field!(reference_black_white, String),
            related_sound_file: extract_field!(related_sound_file, String),
            resolution_unit: extract_field!(resolution_unit, String),
            rows_per_strip: extract_field!(rows_per_strip, parse i32),
            samples_per_pixel: extract_field!(samples_per_pixel, parse i32),
            saturation: extract_field!(saturation, String),
            scene_capture_type: extract_field!(scene_capture_type, String),
            scene_type: extract_field!(scene_type, String),
            sensing_method: extract_field!(sensing_method, String),
            sensitivity_type: extract_field!(sensitivity_type, String),
            sharpness: extract_field!(sharpness, String),
            shutter_speed_value: extract_field!(shutter_speed_value, parse f64),
            software: extract_field!(software, String),
            source_exposure_times_of_composite_image: extract_field!(source_exposure_times_of_composite_image, String),
            source_image_number_of_composite_image: extract_field!(source_image_number_of_composite_image, String),
            spatial_frequency_response: extract_field!(spatial_frequency_response, String),
            spectral_sensitivity: extract_field!(spectral_sensitivity, String),
            standard_output_sensitivity: extract_field!(standard_output_sensitivity, parse i32),
            strip_byte_counts: extract_field!(strip_byte_counts, String),
            strip_offsets: extract_field!(strip_offsets, String),
            sub_sec_time: extract_field!(sub_sec_time, String),
            sub_sec_time_digitized: extract_field!(sub_sec_time_digitized, String),
            sub_sec_time_original: extract_field!(sub_sec_time_original, String),
            subject_area: extract_field!(subject_area, String),
            subject_distance: extract_field!(subject_distance, parse f64),
            subject_distance_range: extract_field!(subject_distance_range, String),
            subject_location: extract_field!(subject_location, String),
            temperature: extract_field!(temperature, parse f64),
            tile_byte_counts: extract_field!(tile_byte_counts, String),
            tile_offsets: extract_field!(tile_offsets, String),
            transfer_function: extract_field!(transfer_function, String),
            user_comment: extract_field!(user_comment, String),
            water_depth: extract_field!(water_depth, parse f64),
            white_balance: extract_field!(white_balance, String),
            white_point: extract_field!(white_point, String),
            x_resolution: extract_field!(x_resolution, parse f64),
            ycbcr_coefficients: extract_field!(ycbcr_coefficients, String),
            ycbcr_positioning: extract_field!(ycbcr_positioning, String),
            ycbcr_sub_sampling: extract_field!(ycbcr_sub_sampling, String),
            y_resolution: extract_field!(y_resolution, parse f64),
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
            "gps_altitude_ref" => self.gps_altitude_ref.clone(),
            "acceleration" => self.acceleration.map(|v| v.to_string()),
            "aperture_value" => self.aperture_value.map(|v| v.to_string()),
            "artist" => self.artist.clone(),
            "body_serial_number" => self.body_serial_number.clone(),
            "bits_per_sample" => self.bits_per_sample.clone(),
            "brightness_value" => self.brightness_value.map(|v| v.to_string()),
            "camera_elevation_angle" => self.camera_elevation_angle.map(|v| v.to_string()),
            "camera_owner_name" => self.camera_owner_name.clone(),
            "cfa_pattern" => self.cfa_pattern.clone(),
            "color_space" => self.color_space.clone(),
            "components_configuration" => self.components_configuration.clone(),
            "composite_image" => self.composite_image.clone(),
            "compressed_bits_per_pixel" => self.compressed_bits_per_pixel.map(|v| v.to_string()),
            "compression" => self.compression.clone(),
            "contrast" => self.contrast.clone(),
            "copyright" => self.copyright.clone(),
            "custom_rendered" => self.custom_rendered.clone(),
            "date_time" => self.date_time.clone(),
            "date_time_original" => self.date_time_original.clone(),
            "date_time_digitized" => self.date_time_digitized.clone(),
            "device_setting_description" => self.device_setting_description.clone(),
            "digital_zoom_ratio" => self.digital_zoom_ratio.map(|v| v.to_string()),
            "exif_version" => self.exif_version.clone(),
            "exposure_bias_value" => self.exposure_bias_value.map(|v| v.to_string()),
            "exposure_index" => self.exposure_index.map(|v| v.to_string()),
            "exposure_mode" => self.exposure_mode.clone(),
            "exposure_program" => self.exposure_program.clone(),
            "file_source" => self.file_source.clone(),
            "flash" => self.flash.clone(),
            "flash_energy" => self.flash_energy.map(|v| v.to_string()),
            "flashpix_version" => self.flashpix_version.clone(),
            "focal_length_in_35mm_film" => self.focal_length_in_35mm_film.map(|v| v.to_string()),
            "focal_plane_resolution_unit" => self.focal_plane_resolution_unit.clone(),
            "focal_plane_x_resolution" => self.focal_plane_x_resolution.map(|v| v.to_string()),
            "focal_plane_y_resolution" => self.focal_plane_y_resolution.map(|v| v.to_string()),
            "gain_control" => self.gain_control.clone(),
            "gamma" => self.gamma.map(|v| v.to_string()),
            "gps_area_information" => self.gps_area_information.clone(),
            "gps_date_stamp" => self.gps_date_stamp.clone(),
            "gps_dest_bearing" => self.gps_dest_bearing.map(|v| v.to_string()),
            "gps_dest_bearing_ref" => self.gps_dest_bearing_ref.clone(),
            "gps_dest_distance" => self.gps_dest_distance.map(|v| v.to_string()),
            "gps_dest_distance_ref" => self.gps_dest_distance_ref.clone(),
            "gps_dest_latitude" => self.gps_dest_latitude.map(|v| v.to_string()),
            "gps_dest_latitude_ref" => self.gps_dest_latitude_ref.clone(),
            "gps_dest_longitude" => self.gps_dest_longitude.map(|v| v.to_string()),
            "gps_dest_longitude_ref" => self.gps_dest_longitude_ref.clone(),
            "gps_differential" => self.gps_differential.clone(),
            "gps_dop" => self.gps_dop.map(|v| v.to_string()),
            "gps_h_positioning_error" => self.gps_h_positioning_error.map(|v| v.to_string()),
            "gps_img_direction" => self.gps_img_direction.map(|v| v.to_string()),
            "gps_img_direction_ref" => self.gps_img_direction_ref.clone(),
            "gps_latitude_ref" => self.gps_latitude_ref.clone(),
            "gps_longitude_ref" => self.gps_longitude_ref.clone(),
            "gps_map_datum" => self.gps_map_datum.clone(),
            "gps_measure_mode" => self.gps_measure_mode.clone(),
            "gps_processing_method" => self.gps_processing_method.clone(),
            "gps_satellites" => self.gps_satellites.clone(),
            "gps_speed" => self.gps_speed.map(|v| v.to_string()),
            "gps_speed_ref" => self.gps_speed_ref.clone(),
            "gps_status" => self.gps_status.clone(),
            "gps_time_stamp" => self.gps_time_stamp.clone(),
            "gps_track" => self.gps_track.map(|v| v.to_string()),
            "gps_track_ref" => self.gps_track_ref.clone(),
            "gps_version_id" => self.gps_version_id.clone(),
            "humidity" => self.humidity.map(|v| v.to_string()),
            "image_description" => self.image_description.clone(),
            "image_unique_id" => self.image_unique_id.clone(),
            "iso_speed_latitude_yyy" => self.iso_speed_latitude_yyy.map(|v| v.to_string()),
            "iso_speed_latitude_zzz" => self.iso_speed_latitude_zzz.map(|v| v.to_string()),
            "jpeg_interchange_format" => self.jpeg_interchange_format.clone(),
            "jpeg_interchange_format_length" => self.jpeg_interchange_format_length.clone(),
            "lens_make" => self.lens_make.clone(),
            "lens_serial_number" => self.lens_serial_number.clone(),
            "lens_specification" => self.lens_specification.clone(),
            "light_source" => self.light_source.clone(),
            "maker_note" => self.maker_note.clone(),
            "max_aperture_value" => self.max_aperture_value.map(|v| v.to_string()),
            "metering_mode" => self.metering_mode.clone(),
            "oecf" => self.oecf.clone(),
            "offset_time" => self.offset_time.clone(),
            "offset_time_digitized" => self.offset_time_digitized.clone(),
            "offset_time_original" => self.offset_time_original.clone(),
            "photometric_interpretation" => self.photometric_interpretation.clone(),
            "pixel_x_dimension" => self.pixel_x_dimension.map(|v| v.to_string()),
            "pixel_y_dimension" => self.pixel_y_dimension.map(|v| v.to_string()),
            "planar_configuration" => self.planar_configuration.clone(),
            "pressure" => self.pressure.map(|v| v.to_string()),
            "primary_chromaticities" => self.primary_chromaticities.clone(),
            "recommended_exposure_index" => self.recommended_exposure_index.map(|v| v.to_string()),
            "reference_black_white" => self.reference_black_white.clone(),
            "related_sound_file" => self.related_sound_file.clone(),
            "resolution_unit" => self.resolution_unit.clone(),
            "rows_per_strip" => self.rows_per_strip.map(|v| v.to_string()),
            "samples_per_pixel" => self.samples_per_pixel.map(|v| v.to_string()),
            "saturation" => self.saturation.clone(),
            "scene_capture_type" => self.scene_capture_type.clone(),
            "scene_type" => self.scene_type.clone(),
            "sensing_method" => self.sensing_method.clone(),
            "sensitivity_type" => self.sensitivity_type.clone(),
            "sharpness" => self.sharpness.clone(),
            "shutter_speed_value" => self.shutter_speed_value.map(|v| v.to_string()),
            "software" => self.software.clone(),
            "source_exposure_times_of_composite_image" => self.source_exposure_times_of_composite_image.clone(),
            "source_image_number_of_composite_image" => self.source_image_number_of_composite_image.clone(),
            "spatial_frequency_response" => self.spatial_frequency_response.clone(),
            "spectral_sensitivity" => self.spectral_sensitivity.clone(),
            "standard_output_sensitivity" => self.standard_output_sensitivity.map(|v| v.to_string()),
            "strip_byte_counts" => self.strip_byte_counts.clone(),
            "strip_offsets" => self.strip_offsets.clone(),
            "sub_sec_time" => self.sub_sec_time.clone(),
            "sub_sec_time_digitized" => self.sub_sec_time_digitized.clone(),
            "sub_sec_time_original" => self.sub_sec_time_original.clone(),
            "subject_area" => self.subject_area.clone(),
            "subject_distance" => self.subject_distance.map(|v| v.to_string()),
            "subject_distance_range" => self.subject_distance_range.clone(),
            "subject_location" => self.subject_location.clone(),
            "temperature" => self.temperature.map(|v| v.to_string()),
            "tile_byte_counts" => self.tile_byte_counts.clone(),
            "tile_offsets" => self.tile_offsets.clone(),
            "transfer_function" => self.transfer_function.clone(),
            "user_comment" => self.user_comment.clone(),
            "water_depth" => self.water_depth.map(|v| v.to_string()),
            "white_balance" => self.white_balance.clone(),
            "white_point" => self.white_point.clone(),
            "x_resolution" => self.x_resolution.map(|v| v.to_string()),
            "ycbcr_coefficients" => self.ycbcr_coefficients.clone(),
            "ycbcr_positioning" => self.ycbcr_positioning.clone(),
            "ycbcr_sub_sampling" => self.ycbcr_sub_sampling.clone(),
            "y_resolution" => self.y_resolution.map(|v| v.to_string()),
            _ => None,
        }
    }

    pub fn set_field(&mut self, key: &str, value: &str) {
        let set_float = |field: &mut Option<f64>| *field = parse_float(value);
        let set_int = |field: &mut Option<i32>| *field = parse_int(value);
        let set_string = |field: &mut Option<String>| *field = if value.is_empty() { None } else { Some(value.to_string()) };

        match key {
            "image_taken_at" => set_string(&mut self.image_taken_at),
            "camera_make" => set_string(&mut self.camera_make),
            "camera_model" => set_string(&mut self.camera_model),
            "lens_model" => set_string(&mut self.lens_model),
            "exposure_time" => set_string(&mut self.exposure_time),
            "f_number" => set_float(&mut self.f_number),
            "iso_speed" => set_int(&mut self.iso_speed),
            "focal_length" => set_float(&mut self.focal_length),
            "width" => set_int(&mut self.width),
            "height" => set_int(&mut self.height),
            "orientation" => set_int(&mut self.orientation),
            "gps_latitude" => set_float(&mut self.gps_latitude),
            "gps_longitude" => set_float(&mut self.gps_longitude),
            "gps_altitude" => set_float(&mut self.gps_altitude),
            "gps_altitude_ref" => set_string(&mut self.gps_altitude_ref),
            "acceleration" => set_float(&mut self.acceleration),
            "aperture_value" => set_float(&mut self.aperture_value),
            "artist" => set_string(&mut self.artist),
            "body_serial_number" => set_string(&mut self.body_serial_number),
            "bits_per_sample" => set_string(&mut self.bits_per_sample),
            "brightness_value" => set_float(&mut self.brightness_value),
            "camera_elevation_angle" => set_float(&mut self.camera_elevation_angle),
            "camera_owner_name" => set_string(&mut self.camera_owner_name),
            "cfa_pattern" => set_string(&mut self.cfa_pattern),
            "color_space" => set_string(&mut self.color_space),
            "components_configuration" => set_string(&mut self.components_configuration),
            "composite_image" => set_string(&mut self.composite_image),
            "compressed_bits_per_pixel" => set_float(&mut self.compressed_bits_per_pixel),
            "compression" => set_string(&mut self.compression),
            "contrast" => set_string(&mut self.contrast),
            "copyright" => set_string(&mut self.copyright),
            "custom_rendered" => set_string(&mut self.custom_rendered),
            "date_time" => set_string(&mut self.date_time),
            "date_time_original" => set_string(&mut self.date_time_original),
            "date_time_digitized" => set_string(&mut self.date_time_digitized),
            "device_setting_description" => set_string(&mut self.device_setting_description),
            "digital_zoom_ratio" => set_float(&mut self.digital_zoom_ratio),
            "exif_version" => set_string(&mut self.exif_version),
            "exposure_bias_value" => set_float(&mut self.exposure_bias_value),
            "exposure_index" => set_float(&mut self.exposure_index),
            "exposure_mode" => set_string(&mut self.exposure_mode),
            "exposure_program" => set_string(&mut self.exposure_program),
            "file_source" => set_string(&mut self.file_source),
            "flash" => set_string(&mut self.flash),
            "flash_energy" => set_float(&mut self.flash_energy),
            "flashpix_version" => set_string(&mut self.flashpix_version),
            "focal_length_in_35mm_film" => set_float(&mut self.focal_length_in_35mm_film),
            "focal_plane_resolution_unit" => set_string(&mut self.focal_plane_resolution_unit),
            "focal_plane_x_resolution" => set_float(&mut self.focal_plane_x_resolution),
            "focal_plane_y_resolution" => set_float(&mut self.focal_plane_y_resolution),
            "gain_control" => set_string(&mut self.gain_control),
            "gamma" => set_float(&mut self.gamma),
            "gps_area_information" => set_string(&mut self.gps_area_information),
            "gps_date_stamp" => set_string(&mut self.gps_date_stamp),
            "gps_dest_bearing" => set_float(&mut self.gps_dest_bearing),
            "gps_dest_bearing_ref" => set_string(&mut self.gps_dest_bearing_ref),
            "gps_dest_distance" => set_float(&mut self.gps_dest_distance),
            "gps_dest_distance_ref" => set_string(&mut self.gps_dest_distance_ref),
            "gps_dest_latitude" => set_float(&mut self.gps_dest_latitude),
            "gps_dest_latitude_ref" => set_string(&mut self.gps_dest_latitude_ref),
            "gps_dest_longitude" => set_float(&mut self.gps_dest_longitude),
            "gps_dest_longitude_ref" => set_string(&mut self.gps_dest_longitude_ref),
            "gps_differential" => set_string(&mut self.gps_differential),
            "gps_dop" => set_float(&mut self.gps_dop),
            "gps_h_positioning_error" => set_float(&mut self.gps_h_positioning_error),
            "gps_img_direction" => set_float(&mut self.gps_img_direction),
            "gps_img_direction_ref" => set_string(&mut self.gps_img_direction_ref),
            "gps_latitude_ref" => set_string(&mut self.gps_latitude_ref),
            "gps_longitude_ref" => set_string(&mut self.gps_longitude_ref),
            "gps_map_datum" => set_string(&mut self.gps_map_datum),
            "gps_measure_mode" => set_string(&mut self.gps_measure_mode),
            "gps_processing_method" => set_string(&mut self.gps_processing_method),
            "gps_satellites" => set_string(&mut self.gps_satellites),
            "gps_speed" => set_float(&mut self.gps_speed),
            "gps_speed_ref" => set_string(&mut self.gps_speed_ref),
            "gps_status" => set_string(&mut self.gps_status),
            "gps_time_stamp" => set_string(&mut self.gps_time_stamp),
            "gps_track" => set_float(&mut self.gps_track),
            "gps_track_ref" => set_string(&mut self.gps_track_ref),
            "gps_version_id" => set_string(&mut self.gps_version_id),
            "humidity" => set_float(&mut self.humidity),
            "image_description" => set_string(&mut self.image_description),
            "image_unique_id" => set_string(&mut self.image_unique_id),
            "iso_speed_latitude_yyy" => set_int(&mut self.iso_speed_latitude_yyy),
            "iso_speed_latitude_zzz" => set_int(&mut self.iso_speed_latitude_zzz),
            "jpeg_interchange_format" => set_string(&mut self.jpeg_interchange_format),
            "jpeg_interchange_format_length" => set_string(&mut self.jpeg_interchange_format_length),
            "lens_make" => set_string(&mut self.lens_make),
            "lens_serial_number" => set_string(&mut self.lens_serial_number),
            "lens_specification" => set_string(&mut self.lens_specification),
            "light_source" => set_string(&mut self.light_source),
            "maker_note" => set_string(&mut self.maker_note),
            "max_aperture_value" => set_float(&mut self.max_aperture_value),
            "metering_mode" => set_string(&mut self.metering_mode),
            "oecf" => set_string(&mut self.oecf),
            "offset_time" => set_string(&mut self.offset_time),
            "offset_time_digitized" => set_string(&mut self.offset_time_digitized),
            "offset_time_original" => set_string(&mut self.offset_time_original),
            "photometric_interpretation" => set_string(&mut self.photometric_interpretation),
            "pixel_x_dimension" => set_int(&mut self.pixel_x_dimension),
            "pixel_y_dimension" => set_int(&mut self.pixel_y_dimension),
            "planar_configuration" => set_string(&mut self.planar_configuration),
            "pressure" => set_float(&mut self.pressure),
            "primary_chromaticities" => set_string(&mut self.primary_chromaticities),
            "recommended_exposure_index" => set_int(&mut self.recommended_exposure_index),
            "reference_black_white" => set_string(&mut self.reference_black_white),
            "related_sound_file" => set_string(&mut self.related_sound_file),
            "resolution_unit" => set_string(&mut self.resolution_unit),
            "rows_per_strip" => set_int(&mut self.rows_per_strip),
            "samples_per_pixel" => set_int(&mut self.samples_per_pixel),
            "saturation" => set_string(&mut self.saturation),
            "scene_capture_type" => set_string(&mut self.scene_capture_type),
            "scene_type" => set_string(&mut self.scene_type),
            "sensing_method" => set_string(&mut self.sensing_method),
            "sensitivity_type" => set_string(&mut self.sensitivity_type),
            "sharpness" => set_string(&mut self.sharpness),
            "shutter_speed_value" => set_float(&mut self.shutter_speed_value),
            "software" => set_string(&mut self.software),
            "source_exposure_times_of_composite_image" => set_string(&mut self.source_exposure_times_of_composite_image),
            "source_image_number_of_composite_image" => set_string(&mut self.source_image_number_of_composite_image),
            "spatial_frequency_response" => set_string(&mut self.spatial_frequency_response),
            "spectral_sensitivity" => set_string(&mut self.spectral_sensitivity),
            "standard_output_sensitivity" => set_int(&mut self.standard_output_sensitivity),
            "strip_byte_counts" => set_string(&mut self.strip_byte_counts),
            "strip_offsets" => set_string(&mut self.strip_offsets),
            "sub_sec_time" => set_string(&mut self.sub_sec_time),
            "sub_sec_time_digitized" => set_string(&mut self.sub_sec_time_digitized),
            "sub_sec_time_original" => set_string(&mut self.sub_sec_time_original),
            "subject_area" => set_string(&mut self.subject_area),
            "subject_distance" => set_float(&mut self.subject_distance),
            "subject_distance_range" => set_string(&mut self.subject_distance_range),
            "subject_location" => set_string(&mut self.subject_location),
            "temperature" => set_float(&mut self.temperature),
            "tile_byte_counts" => set_string(&mut self.tile_byte_counts),
            "tile_offsets" => set_string(&mut self.tile_offsets),
            "transfer_function" => set_string(&mut self.transfer_function),
            "user_comment" => set_string(&mut self.user_comment),
            "water_depth" => set_float(&mut self.water_depth),
            "white_balance" => set_string(&mut self.white_balance),
            "white_point" => set_string(&mut self.white_point),
            "x_resolution" => set_float(&mut self.x_resolution),
            "ycbcr_coefficients" => set_string(&mut self.ycbcr_coefficients),
            "ycbcr_positioning" => set_string(&mut self.ycbcr_positioning),
            "ycbcr_sub_sampling" => set_string(&mut self.ycbcr_sub_sampling),
            "y_resolution" => set_float(&mut self.y_resolution),
            _ => {}
        }
    }

    pub fn set_field_by_tag(&mut self, tag: &exif::Tag, value: &str) {
        // Helper closures for safe parsing
        let parse_float = || value.parse().ok();
        let parse_int = || value.parse().ok();
        let as_string = || if value.is_empty() { None } else { Some(value.to_string()) };

        match tag {
            &exif::Tag::Acceleration => self.acceleration = parse_float(),
            &exif::Tag::ApertureValue => self.aperture_value = parse_float(),
            &exif::Tag::Artist => self.artist = as_string(),
            &exif::Tag::BodySerialNumber => self.body_serial_number = as_string(),
            &exif::Tag::BitsPerSample => self.bits_per_sample = as_string(),
            &exif::Tag::BrightnessValue => self.brightness_value = parse_float(),
            &exif::Tag::CameraElevationAngle => self.camera_elevation_angle = parse_float(),
            &exif::Tag::CameraOwnerName => self.camera_owner_name = as_string(),
            &exif::Tag::CFAPattern => self.cfa_pattern = as_string(),
            &exif::Tag::ColorSpace => self.color_space = as_string(),
            &exif::Tag::ComponentsConfiguration => self.components_configuration = as_string(),
            &exif::Tag::CompositeImage => self.composite_image = as_string(),
            &exif::Tag::CompressedBitsPerPixel => self.compressed_bits_per_pixel = parse_float(),
            &exif::Tag::Compression => self.compression = as_string(),
            &exif::Tag::Contrast => self.contrast = as_string(),
            &exif::Tag::Copyright => self.copyright = as_string(),
            &exif::Tag::CustomRendered => self.custom_rendered = as_string(),
            &exif::Tag::DateTime => self.date_time = as_string(),
            &exif::Tag::DateTimeDigitized => self.date_time_digitized = as_string(),
            &exif::Tag::DateTimeOriginal => self.date_time_original = as_string(),
            &exif::Tag::DeviceSettingDescription => self.device_setting_description = as_string(),
            &exif::Tag::DigitalZoomRatio => self.digital_zoom_ratio = parse_float(),
            &exif::Tag::ExifVersion => self.exif_version = as_string(),
            &exif::Tag::ExposureBiasValue => self.exposure_bias_value = parse_float(),
            &exif::Tag::ExposureIndex => self.exposure_index = parse_float(),
            &exif::Tag::ExposureMode => self.exposure_mode = as_string(),
            &exif::Tag::ExposureProgram => self.exposure_program = as_string(),
            &exif::Tag::ExposureTime => self.exposure_time = as_string(),
            &exif::Tag::FileSource => self.file_source = as_string(),
            &exif::Tag::Flash => self.flash = as_string(),
            &exif::Tag::FlashEnergy => self.flash_energy = parse_float(),
            &exif::Tag::FlashpixVersion => self.flashpix_version = as_string(),
            &exif::Tag::FNumber => self.f_number = parse_float(),
            &exif::Tag::FocalLength => self.focal_length = parse_float(),
            &exif::Tag::FocalLengthIn35mmFilm => self.focal_length_in_35mm_film = parse_float(),
            &exif::Tag::FocalPlaneResolutionUnit => self.focal_plane_resolution_unit = as_string(),
            &exif::Tag::FocalPlaneXResolution => self.focal_plane_x_resolution = parse_float(),
            &exif::Tag::FocalPlaneYResolution => self.focal_plane_y_resolution = parse_float(),
            &exif::Tag::GainControl => self.gain_control = as_string(),
            &exif::Tag::Gamma => self.gamma = parse_float(),
            &exif::Tag::GPSAltitude => self.gps_altitude = parse_float(),
            &exif::Tag::GPSAltitudeRef => self.gps_altitude_ref = as_string(),
            &exif::Tag::GPSAreaInformation => self.gps_area_information = as_string(),
            &exif::Tag::GPSDateStamp => self.gps_date_stamp = as_string(),
            &exif::Tag::GPSDestBearing => self.gps_dest_bearing = parse_float(),
            &exif::Tag::GPSDestBearingRef => self.gps_dest_bearing_ref = as_string(),
            &exif::Tag::GPSDestDistance => self.gps_dest_distance = parse_float(),
            &exif::Tag::GPSDestDistanceRef => self.gps_dest_distance_ref = as_string(),
            &exif::Tag::GPSDestLatitude => self.gps_dest_latitude = parse_float(),
            &exif::Tag::GPSDestLatitudeRef => self.gps_dest_latitude_ref = as_string(),
            &exif::Tag::GPSDestLongitude => self.gps_dest_longitude = parse_float(),
            &exif::Tag::GPSDestLongitudeRef => self.gps_dest_longitude_ref = as_string(),
            &exif::Tag::GPSDifferential => self.gps_differential = as_string(),
            &exif::Tag::GPSDOP => self.gps_dop = parse_float(),
            &exif::Tag::GPSHPositioningError => self.gps_h_positioning_error = parse_float(),
            &exif::Tag::GPSImgDirection => self.gps_img_direction = parse_float(),
            &exif::Tag::GPSImgDirectionRef => self.gps_img_direction_ref = as_string(),
            &exif::Tag::GPSLatitude => self.gps_latitude = parse_float(),
            &exif::Tag::GPSLatitudeRef => self.gps_latitude_ref = as_string(),
            &exif::Tag::GPSLongitude => self.gps_longitude = parse_float(),
            &exif::Tag::GPSLongitudeRef => self.gps_longitude_ref = as_string(),
            &exif::Tag::GPSMapDatum => self.gps_map_datum = as_string(),
            &exif::Tag::GPSMeasureMode => self.gps_measure_mode = as_string(),
            &exif::Tag::GPSProcessingMethod => self.gps_processing_method = as_string(),
            &exif::Tag::GPSSatellites => self.gps_satellites = as_string(),
            &exif::Tag::GPSSpeed => self.gps_speed = parse_float(),
            &exif::Tag::GPSSpeedRef => self.gps_speed_ref = as_string(),
            &exif::Tag::GPSStatus => self.gps_status = as_string(),
            &exif::Tag::GPSTimeStamp => self.gps_time_stamp = as_string(),
            &exif::Tag::GPSTrack => self.gps_track = parse_float(),
            &exif::Tag::GPSTrackRef => self.gps_track_ref = as_string(),
            &exif::Tag::GPSVersionID => self.gps_version_id = as_string(),
            &exif::Tag::Humidity => self.humidity = parse_float(),
            &exif::Tag::ImageDescription => self.image_description = as_string(),
            &exif::Tag::ImageLength => self.height = parse_int(), // ImageLength maps to height
            &exif::Tag::ImageUniqueID => self.image_unique_id = as_string(),
            &exif::Tag::ImageWidth => self.width = parse_int(), // ImageWidth maps to width
            &exif::Tag::ISOSpeed => self.iso_speed = parse_int(),
            &exif::Tag::ISOSpeedLatitudeyyy => self.iso_speed_latitude_yyy = parse_int(),
            &exif::Tag::ISOSpeedLatitudezzz => self.iso_speed_latitude_zzz = parse_int(),
            &exif::Tag::JPEGInterchangeFormat => self.jpeg_interchange_format = as_string(),
            &exif::Tag::JPEGInterchangeFormatLength => self.jpeg_interchange_format_length = as_string(),
            &exif::Tag::LensMake => self.lens_make = as_string(),
            &exif::Tag::LensModel => self.lens_model = as_string(),
            &exif::Tag::LensSerialNumber => self.lens_serial_number = as_string(),
            &exif::Tag::LensSpecification => self.lens_specification = as_string(),
            &exif::Tag::LightSource => self.light_source = as_string(),
            &exif::Tag::Make => self.camera_make = as_string(), // Make maps to camera_make
            &exif::Tag::MakerNote => self.maker_note = as_string(),
            &exif::Tag::MaxApertureValue => self.max_aperture_value = parse_float(),
            &exif::Tag::MeteringMode => self.metering_mode = as_string(),
            &exif::Tag::Model => self.camera_model = as_string(), // Model maps to camera_model
            &exif::Tag::OECF => self.oecf = as_string(),
            &exif::Tag::OffsetTime => self.offset_time = as_string(),
            &exif::Tag::OffsetTimeDigitized => self.offset_time_digitized = as_string(),
            &exif::Tag::OffsetTimeOriginal => self.offset_time_original = as_string(),
            &exif::Tag::Orientation => self.orientation = parse_int(),
            &exif::Tag::PhotometricInterpretation => self.photometric_interpretation = as_string(),
            &exif::Tag::PhotographicSensitivity => self.iso_speed = parse_int(), // PhotographicSensitivity maps to iso_speed
            &exif::Tag::PixelXDimension => self.pixel_x_dimension = parse_int(),
            &exif::Tag::PixelYDimension => self.pixel_y_dimension = parse_int(),
            &exif::Tag::PlanarConfiguration => self.planar_configuration = as_string(),
            &exif::Tag::Pressure => self.pressure = parse_float(),
            &exif::Tag::PrimaryChromaticities => self.primary_chromaticities = as_string(),
            &exif::Tag::RecommendedExposureIndex => self.recommended_exposure_index = parse_int(),
            &exif::Tag::ReferenceBlackWhite => self.reference_black_white = as_string(),
            &exif::Tag::RelatedSoundFile => self.related_sound_file = as_string(),
            &exif::Tag::ResolutionUnit => self.resolution_unit = as_string(),
            &exif::Tag::RowsPerStrip => self.rows_per_strip = parse_int(),
            &exif::Tag::SamplesPerPixel => self.samples_per_pixel = parse_int(),
            &exif::Tag::Saturation => self.saturation = as_string(),
            &exif::Tag::SceneCaptureType => self.scene_capture_type = as_string(),
            &exif::Tag::SceneType => self.scene_type = as_string(),
            &exif::Tag::SensingMethod => self.sensing_method = as_string(),
            &exif::Tag::SensitivityType => self.sensitivity_type = as_string(),
            &exif::Tag::Sharpness => self.sharpness = as_string(),
            &exif::Tag::ShutterSpeedValue => self.shutter_speed_value = parse_float(),
            &exif::Tag::Software => self.software = as_string(),
            &exif::Tag::SourceExposureTimesOfCompositeImage => self.source_exposure_times_of_composite_image = as_string(),
            &exif::Tag::SourceImageNumberOfCompositeImage => self.source_image_number_of_composite_image = as_string(),
            &exif::Tag::SpatialFrequencyResponse => self.spatial_frequency_response = as_string(),
            &exif::Tag::SpectralSensitivity => self.spectral_sensitivity = as_string(),
            &exif::Tag::StandardOutputSensitivity => self.standard_output_sensitivity = parse_int(),
            &exif::Tag::StripByteCounts => self.strip_byte_counts = as_string(),
            &exif::Tag::StripOffsets => self.strip_offsets = as_string(),
            &exif::Tag::SubSecTime => self.sub_sec_time = as_string(),
            &exif::Tag::SubSecTimeDigitized => self.sub_sec_time_digitized = as_string(),
            &exif::Tag::SubSecTimeOriginal => self.sub_sec_time_original = as_string(),
            &exif::Tag::SubjectArea => self.subject_area = as_string(),
            &exif::Tag::SubjectDistance => self.subject_distance = parse_float(),
            &exif::Tag::SubjectDistanceRange => self.subject_distance_range = as_string(),
            &exif::Tag::SubjectLocation => self.subject_location = as_string(),
            &exif::Tag::Temperature => self.temperature = parse_float(),
            &exif::Tag::TileByteCounts => self.tile_byte_counts = as_string(),
            &exif::Tag::TileOffsets => self.tile_offsets = as_string(),
            &exif::Tag::TransferFunction => self.transfer_function = as_string(),
            &exif::Tag::UserComment => self.user_comment = as_string(),
            &exif::Tag::WaterDepth => self.water_depth = parse_float(),
            &exif::Tag::WhiteBalance => self.white_balance = as_string(),
            &exif::Tag::WhitePoint => self.white_point = as_string(),
            &exif::Tag::XResolution => self.x_resolution = parse_float(),
            &exif::Tag::YCbCrCoefficients => self.ycbcr_coefficients = as_string(),
            &exif::Tag::YCbCrPositioning => self.ycbcr_positioning = as_string(),
            &exif::Tag::YCbCrSubSampling => self.ycbcr_sub_sampling = as_string(),
            &exif::Tag::YResolution => self.y_resolution = parse_float(),
            _ => {
                // Instead of panicking, log the unsupported tag for debugging
                // match tag.to_string().as_str() {
                //     exif::Tag::InteroperabilityIndex => {},
                //     _ => {
                //         eprintln!("Unsupported EXIF tag ({}): {:?}", tag.to_string().as_str(), tag);
                //     }
                // }
            }
        }
    }
    
    pub fn get_meta() -> Vec<ImageFieldMeta> {
        serde_json::from_str::<Vec<ImageFieldMeta>>(IMAGE_EXIF_COLUMNS_JSON).unwrap()
    }
    
    pub fn default(image_path: &str) -> Self {
        Self {
            image_path: image_path.to_string(),
            image_taken_at: None,
            camera_make: None,
            camera_model: None,
            lens_model: None,
            exposure_time: None,
            f_number: None,
            iso_speed: None,
            focal_length: None,
            width: None,
            height: None,
            orientation: None,
            gps_latitude: None,
            gps_longitude: None,
            gps_altitude: None,
            gps_altitude_ref: None,
            acceleration: None,
            aperture_value: None,
            artist: None,
            body_serial_number: None,
            bits_per_sample: None,
            brightness_value: None,
            camera_elevation_angle: None,
            camera_owner_name: None,
            cfa_pattern: None,
            color_space: None,
            components_configuration: None,
            composite_image: None,
            compressed_bits_per_pixel: None,
            compression: None,
            contrast: None,
            copyright: None,
            custom_rendered: None,
            date_time: None,
            date_time_original: None,
            date_time_digitized: None,
            device_setting_description: None,
            digital_zoom_ratio: None,
            exif_version: None,
            exposure_bias_value: None,
            exposure_index: None,
            exposure_mode: None,
            exposure_program: None,
            file_source: None,
            flash: None,
            flash_energy: None,
            flashpix_version: None,
            focal_length_in_35mm_film: None,
            focal_plane_resolution_unit: None,
            focal_plane_x_resolution: None,
            focal_plane_y_resolution: None,
            gain_control: None,
            gamma: None,
            gps_area_information: None,
            gps_date_stamp: None,
            gps_dest_bearing: None,
            gps_dest_bearing_ref: None,
            gps_dest_distance: None,
            gps_dest_distance_ref: None,
            gps_dest_latitude: None,
            gps_dest_latitude_ref: None,
            gps_dest_longitude: None,
            gps_dest_longitude_ref: None,
            gps_differential: None,
            gps_dop: None,
            gps_h_positioning_error: None,
            gps_img_direction: None,
            gps_img_direction_ref: None,
            gps_latitude_ref: None,
            gps_longitude_ref: None,
            gps_map_datum: None,
            gps_measure_mode: None,
            gps_processing_method: None,
            gps_satellites: None,
            gps_speed: None,
            gps_speed_ref: None,
            gps_status: None,
            gps_time_stamp: None,
            gps_track: None,
            gps_track_ref: None,
            gps_version_id: None,
            humidity: None,
            image_description: None,
            image_unique_id: None,
            iso_speed_latitude_yyy: None,
            iso_speed_latitude_zzz: None,
            jpeg_interchange_format: None,
            jpeg_interchange_format_length: None,
            lens_make: None,
            lens_serial_number: None,
            lens_specification: None,
            light_source: None,
            maker_note: None,
            max_aperture_value: None,
            metering_mode: None,
            oecf: None,
            offset_time: None,
            offset_time_digitized: None,
            offset_time_original: None,
            photometric_interpretation: None,
            pixel_x_dimension: None,
            pixel_y_dimension: None,
            planar_configuration: None,
            pressure: None,
            primary_chromaticities: None,
            recommended_exposure_index: None,
            reference_black_white: None,
            related_sound_file: None,
            resolution_unit: None,
            rows_per_strip: None,
            samples_per_pixel: None,
            saturation: None,
            scene_capture_type: None,
            scene_type: None,
            sensing_method: None,
            sensitivity_type: None,
            sharpness: None,
            shutter_speed_value: None,
            software: None,
            source_exposure_times_of_composite_image: None,
            source_image_number_of_composite_image: None,
            spatial_frequency_response: None,
            spectral_sensitivity: None,
            standard_output_sensitivity: None,
            strip_byte_counts: None,
            strip_offsets: None,
            sub_sec_time: None,
            sub_sec_time_digitized: None,
            sub_sec_time_original: None,
            subject_area: None,
            subject_distance: None,
            subject_distance_range: None,
            subject_location: None,
            temperature: None,
            tile_byte_counts: None,
            tile_offsets: None,
            transfer_function: None,
            user_comment: None,
            water_depth: None,
            white_balance: None,
            white_point: None,
            x_resolution: None,
            ycbcr_coefficients: None,
            ycbcr_positioning: None,
            ycbcr_sub_sampling: None,
            y_resolution: None,
        }
    }
}

impl std::fmt::Display for ImageExif {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "image_taken_at: {:?}, ", self.image_taken_at)?;
        write!(f, "camera_make: {:?}, , ", self.camera_make)?;
        write!(f, "orientation: {:?}, ", self.orientation)?;
        write!(f, "image_taken_at: {:?}, ", self.image_taken_at)?;
        write!(f, "camera_make: {:?}, ", self.camera_make)?;
        write!(f, "camera_model: {:?}, ", self.camera_model)?;
        write!(f, "lens_model: {:?}, ", self.lens_model)?;
        write!(f, "exposure_time: {:?}, ", self.exposure_time)?;
        write!(f, "f_number: {:?}, ", self.f_number)?;
        write!(f, "iso_speed: {:?}, ", self.iso_speed)?;
        write!(f, "focal_length: {:?}, ", self.focal_length)?;
        write!(f, "width: {:?}, ", self.width)?;
        write!(f, "height: {:?}, ", self.height)?;
        write!(f, "orientation: {:?}, ", self.orientation)?;
        write!(f, "gps_latitude: {:?}, ", self.gps_latitude)?;
        write!(f, "gps_longitude: {:?}, ", self.gps_longitude)?;
        write!(f, "gps_altitude: {:?}, ", self.gps_altitude)?;
        write!(f, "gps_altitude_ref: {:?}, ", self.gps_altitude_ref)?;
        write!(f, "acceleration: {:?}, ", self.acceleration)?;
        write!(f, "aperture_value: {:?}, ", self.aperture_value)?;
        write!(f, "artist: {:?}, ", self.artist)?;
        write!(f, "body_serial_number: {:?}, ", self.body_serial_number)?;
        write!(f, "bits_per_sample: {:?}, ", self.bits_per_sample)?;
        write!(f, "brightness_value: {:?}, ", self.brightness_value)?;
        write!(f, "camera_elevation_angle: {:?}, ", self.camera_elevation_angle)?;
        write!(f, "camera_owner_name: {:?}, ", self.camera_owner_name)?;
        write!(f, "cfa_pattern: {:?}, ", self.cfa_pattern)?;
        write!(f, "color_space: {:?}, ", self.color_space)?;
        write!(f, "components_configuration: {:?}, ", self.components_configuration)?;
        write!(f, "composite_image: {:?}, ", self.composite_image)?;
        write!(f, "compressed_bits_per_pixel: {:?}, ", self.compressed_bits_per_pixel)?;
        write!(f, "compression: {:?}, ", self.compression)?;
        write!(f, "contrast: {:?}, ", self.contrast)?;
        write!(f, "copyright: {:?}, ", self.copyright)?;
        write!(f, "custom_rendered: {:?}, ", self.custom_rendered)?;
        write!(f, "date_time: {:?}, ", self.date_time)?;
        write!(f, "date_time_original: {:?}, ", self.date_time_original)?;
        write!(f, "date_time_digitized: {:?}, ", self.date_time_digitized)?;
        write!(f, "device_setting_description: {:?}, ", self.device_setting_description)?;
        write!(f, "digital_zoom_ratio: {:?}, ", self.digital_zoom_ratio)?;
        write!(f, "exif_version: {:?}, ", self.exif_version)?;
        write!(f, "exposure_bias_value: {:?}, ", self.exposure_bias_value)?;
        write!(f, "exposure_index: {:?}, ", self.exposure_index)?;
        write!(f, "exposure_mode: {:?}, ", self.exposure_mode)?;
        write!(f, "exposure_program: {:?}, ", self.exposure_program)?;
        write!(f, "file_source: {:?}, ", self.file_source)?;
        write!(f, "flash: {:?}, ", self.flash)?;
        write!(f, "flash_energy: {:?}, ", self.flash_energy)?;
        write!(f, "flashpix_version: {:?}, ", self.flashpix_version)?;
        write!(f, "focal_length_in_35mm_film: {:?}, ", self.focal_length_in_35mm_film)?;
        write!(f, "focal_plane_resolution_unit: {:?}, ", self.focal_plane_resolution_unit)?;
        write!(f, "focal_plane_x_resolution: {:?}, ", self.focal_plane_x_resolution)?;
        write!(f, "focal_plane_y_resolution: {:?}, ", self.focal_plane_y_resolution)?;
        write!(f, "gain_control: {:?}, ", self.gain_control)?;
        write!(f, "gamma: {:?}, ", self.gamma)?;
        write!(f, "gps_area_information: {:?}, ", self.gps_area_information)?;
        write!(f, "gps_date_stamp: {:?}, ", self.gps_date_stamp)?;
        write!(f, "gps_dest_bearing: {:?}, ", self.gps_dest_bearing)?;
        write!(f, "gps_dest_bearing_ref: {:?}, ", self.gps_dest_bearing_ref)?;
        write!(f, "gps_dest_distance: {:?}, ", self.gps_dest_distance)?;
        write!(f, "gps_dest_distance_ref: {:?}, ", self.gps_dest_distance_ref)?;
        write!(f, "gps_dest_latitude: {:?}, ", self.gps_dest_latitude)?;
        write!(f, "gps_dest_latitude_ref: {:?}, ", self.gps_dest_latitude_ref)?;
        write!(f, "gps_dest_longitude: {:?}, ", self.gps_dest_longitude)?;
        write!(f, "gps_dest_longitude_ref: {:?}, ", self.gps_dest_longitude_ref)?;
        write!(f, "gps_differential: {:?}, ", self.gps_differential)?;
        write!(f, "gps_dop: {:?}, ", self.gps_dop)?;
        write!(f, "gps_h_positioning_error: {:?}, ", self.gps_h_positioning_error)?;
        write!(f, "gps_img_direction: {:?}, ", self.gps_img_direction)?;
        write!(f, "gps_img_direction_ref: {:?}, ", self.gps_img_direction_ref)?;
        write!(f, "gps_latitude_ref: {:?}, ", self.gps_latitude_ref)?;
        write!(f, "gps_longitude_ref: {:?}, ", self.gps_longitude_ref)?;
        write!(f, "gps_map_datum: {:?}, ", self.gps_map_datum)?;
        write!(f, "gps_measure_mode: {:?}, ", self.gps_measure_mode)?;
        write!(f, "gps_processing_method: {:?}, ", self.gps_processing_method)?;
        write!(f, "gps_satellites: {:?}, ", self.gps_satellites)?;
        write!(f, "gps_speed: {:?}, ", self.gps_speed)?;
        write!(f, "gps_speed_ref: {:?}, ", self.gps_speed_ref)?;
        write!(f, "gps_status: {:?}, ", self.gps_status)?;
        write!(f, "gps_time_stamp: {:?}, ", self.gps_time_stamp)?;
        write!(f, "gps_track: {:?}, ", self.gps_track)?;
        write!(f, "gps_track_ref: {:?}, ", self.gps_track_ref)?;
        write!(f, "gps_version_id: {:?}, ", self.gps_version_id)?;
        write!(f, "humidity: {:?}, ", self.humidity)?;
        write!(f, "image_description: {:?}, ", self.image_description)?;
        write!(f, "image_unique_id: {:?}, ", self.image_unique_id)?;
        write!(f, "iso_speed_latitude_yyy: {:?}, ", self.iso_speed_latitude_yyy)?;
        write!(f, "iso_speed_latitude_zzz: {:?}, ", self.iso_speed_latitude_zzz)?;
        write!(f, "jpeg_interchange_format: {:?}, ", self.jpeg_interchange_format)?;
        write!(f, "jpeg_interchange_format_length: {:?}, ", self.jpeg_interchange_format_length)?;
        write!(f, "lens_make: {:?}, ", self.lens_make)?;
        write!(f, "lens_serial_number: {:?}, ", self.lens_serial_number)?;
        write!(f, "lens_specification: {:?}, ", self.lens_specification)?;
        write!(f, "light_source: {:?}, ", self.light_source)?;
        write!(f, "maker_note: {:?}, ", self.maker_note)?;
        write!(f, "max_aperture_value: {:?}, ", self.max_aperture_value)?;
        write!(f, "metering_mode: {:?}, ", self.metering_mode)?;
        write!(f, "oecf: {:?}, ", self.oecf)?;
        write!(f, "offset_time: {:?}, ", self.offset_time)?;
        write!(f, "offset_time_digitized: {:?}, ", self.offset_time_digitized)?;
        write!(f, "offset_time_original: {:?}, ", self.offset_time_original)?;
        write!(f, "photometric_interpretation: {:?}, ", self.photometric_interpretation)?;
        write!(f, "pixel_x_dimension: {:?}, ", self.pixel_x_dimension)?;
        write!(f, "pixel_y_dimension: {:?}, ", self.pixel_y_dimension)?;
        write!(f, "planar_configuration: {:?}, ", self.planar_configuration)?;
        write!(f, "pressure: {:?}, ", self.pressure)?;
        write!(f, "primary_chromaticities: {:?}, ", self.primary_chromaticities)?;
        write!(f, "recommended_exposure_index: {:?}, ", self.recommended_exposure_index)?;
        write!(f, "reference_black_white: {:?}, ", self.reference_black_white)?;
        write!(f, "related_sound_file: {:?}, ", self.related_sound_file)?;
        write!(f, "resolution_unit: {:?}, ", self.resolution_unit)?;
        write!(f, "rows_per_strip: {:?}, ", self.rows_per_strip)?;
        write!(f, "samples_per_pixel: {:?}, ", self.samples_per_pixel)?;
        write!(f, "saturation: {:?}, ", self.saturation)?;
        write!(f, "scene_capture_type: {:?}, ", self.scene_capture_type)?;
        write!(f, "scene_type: {:?}, ", self.scene_type)?;
        write!(f, "sensing_method: {:?}, ", self.sensing_method)?;
        write!(f, "sensitivity_type: {:?}, ", self.sensitivity_type)?;
        write!(f, "sharpness: {:?}, ", self.sharpness)?;
        write!(f, "shutter_speed_value: {:?}, ", self.shutter_speed_value)?;
        write!(f, "software: {:?}, ", self.software)?;
        write!(f, "source_exposure_times_of_composite_image: {:?}, ", self.source_exposure_times_of_composite_image)?;
        write!(f, "source_image_number_of_composite_image: {:?}, ", self.source_image_number_of_composite_image)?;
        write!(f, "spatial_frequency_response: {:?}, ", self.spatial_frequency_response)?;
        write!(f, "spectral_sensitivity: {:?}, ", self.spectral_sensitivity)?;
        write!(f, "standard_output_sensitivity: {:?}, ", self.standard_output_sensitivity)?;
        write!(f, "strip_byte_counts: {:?}, ", self.strip_byte_counts)?;
        write!(f, "strip_offsets: {:?}, ", self.strip_offsets)?;
        write!(f, "sub_sec_time: {:?}, ", self.sub_sec_time)?;
        write!(f, "sub_sec_time_digitized: {:?}, ", self.sub_sec_time_digitized)?;
        write!(f, "sub_sec_time_original: {:?}, ", self.sub_sec_time_original)?;
        write!(f, "subject_area: {:?}, ", self.subject_area)?;
        write!(f, "subject_distance: {:?}, ", self.subject_distance)?;
        write!(f, "subject_distance_range: {:?}, ", self.subject_distance_range)?;
        write!(f, "subject_location: {:?}, ", self.subject_location)?;
        write!(f, "temperature: {:?}, ", self.temperature)?;
        write!(f, "tile_byte_counts: {:?}, ", self.tile_byte_counts)?;
        write!(f, "tile_offsets: {:?}, ", self.tile_offsets)?;
        write!(f, "transfer_function: {:?}, ", self.transfer_function)?;
        write!(f, "user_comment: {:?}, ", self.user_comment)?;
        write!(f, "water_depth: {:?}, ", self.water_depth)?;
        write!(f, "white_balance: {:?}, ", self.white_balance)?;
        write!(f, "white_point: {:?}, ", self.white_point)?;
        write!(f, "x_resolution: {:?}, ", self.x_resolution)?;
        write!(f, "ycbcr_coefficients: {:?}, ", self.ycbcr_coefficients)?;
        write!(f, "ycbcr_positioning: {:?}, ", self.ycbcr_positioning)?;
        write!(f, "ycbcr_sub_sampling: {:?}, ", self.ycbcr_sub_sampling)?;
        write!(f, "y_resolution: {:?}, ", self.y_resolution)?;
        Ok(())
    }
}