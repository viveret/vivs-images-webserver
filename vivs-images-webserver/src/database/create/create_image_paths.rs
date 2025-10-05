
pub const SQL_CREATE_IMAGE_PATHS: &str = r#"
CREATE TABLE IF NOT EXISTS image_paths (
    image_path TEXT PRIMARY KEY
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_image_paths_image_path ON image_paths(image_path);

"#;