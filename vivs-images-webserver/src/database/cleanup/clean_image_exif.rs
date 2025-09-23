
pub const CLEAN_IMAGE_EXIF_SQL: &str = r#"
-- @name: CLEAN_IMAGE_EXIF_SQL
-- @label: Clean Image Exif Table
-- @description: Removes exif entries
-- @is_runnable: true

DELETE FROM image_exif;

"#;