
pub const SQL_CREATE_IMAGE_SIMILARITY: &str = r#"
CREATE TABLE IF NOT EXISTS image_similarity (
    image_comparison_key INTEGER NOT NULL PRIMARY KEY,
    image_comparison_algorithm: INTEGER NOT NULL,
    image_path_a TEXT,
    image_path_b TEXT,
    similarity_value REAL NOT NULL,
    similarity_confidence REAL NOT NULL,
);

CREATE INDEX IF NOT EXISTS idx_image_similarity_comparison_key ON image_similarity(image_comparison_key);
CREATE UNIQUE INDEX IF NOT EXISTS idx_image_similarity_image_path ON image_similarity(image_path_a, image_path_b);

"#;