
pub const SQL_CREATE_IMAGE_BRIGHTNESS: &str = r#"
CREATE TABLE IF NOT EXISTS image_brightness (
    image_path TEXT PRIMARY KEY,
    brightness REAL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_image_path ON image_brightness(image_path);

"#;