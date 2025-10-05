
pub const SQL_CREATE_IMAGE_XMP: &str = r#"
CREATE TABLE IF NOT EXISTS image_xmp (
    image_path TEXT PRIMARY KEY,
    xmp TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_image_xmp_image_path ON image_xmp(image_path);

"#;