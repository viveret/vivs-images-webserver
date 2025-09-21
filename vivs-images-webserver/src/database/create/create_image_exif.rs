
pub const SQL_CREATE_IMAGE_EXIF: &str = r#"
CREATE TABLE IF NOT EXISTS image_exif (
    image_path TEXT PRIMARY KEY,
    image_taken_at TIMESTAMP NULL,
    camera_make TEXT NULL,
    camera_model TEXT NULL,
    lens_model TEXT NULL,
    exposure_time TEXT NULL,
    f_number REAL NULL,
    iso_speed INTEGER NULL,
    focal_length REAL NULL,
    width INTEGER NULL,
    height INTEGER NULL,
    orientation INTEGER NULL,
    gps_latitude REAL NULL,
    gps_longitude REAL NULL,
    gps_altitude REAL NULL
);

CREATE INDEX IF NOT EXISTS idx_image_path ON image_exif(image_path);
CREATE INDEX IF NOT EXISTS idx_taken_at ON image_exif(image_taken_at);

"#;