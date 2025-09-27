
pub const SQL_CREATE_IMAGE_THUMBNAIL: &str = r#"
CREATE TABLE IF NOT EXISTS image_thumbnail (
    image_thumbnail_key INTEGER NOT NULL PRIMARY KEY,
    image_path TEXT NOT NULL,
    width_and_length INTEGER NOT NULL,
    thumbnail_format INTEGER NOT NULL,
    thumbnail_data BLOB NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_image_thumbnail_key ON image_thumbnail(image_thumbnail_key);
CREATE UNIQUE INDEX IF NOT EXISTS idx_image_thumbnail_image_path_width_and_length ON image_thumbnail(image_path, width_and_length);
CREATE INDEX IF NOT EXISTS idx_image_thumbnail_image_path ON image_thumbnail(image_path);

"#;