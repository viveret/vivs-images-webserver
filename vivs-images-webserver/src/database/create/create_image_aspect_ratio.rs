
pub const SQL_CREATE_IMAGE_ASPECT_RATIO: &str = r#"
CREATE TABLE IF NOT EXISTS image_aspect_ratio (
    image_path TEXT PRIMARY KEY,
    width_pixels INTEGER NOT NULL,
    height_pixels INTEGER NOT NULL,
    aspect_ratio REAL NOT NULL,
    quality INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_image_path ON image_aspect_ratio(image_path);

"#;