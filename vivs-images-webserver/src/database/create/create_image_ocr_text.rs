
pub const SQL_CREATE_IMAGE_OCR_TEXT: &str = r#"
CREATE TABLE IF NOT EXISTS image_ocr_text (
    image_path TEXT PRIMARY KEY,
    ocr_text TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_image_path ON image_ocr_text(image_path);
CREATE INDEX IF NOT EXISTS idx_ocr_text ON image_ocr_text(ocr_text);

"#;