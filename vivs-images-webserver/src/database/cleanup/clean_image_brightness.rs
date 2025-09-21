
pub const CLEAN_IMAGE_BRIGHTNESS_SQL: &str = r#"
-- @name: CLEAN_IMAGE_BRIGHTNESS_SQL
-- @label: Clean Image Brightness Table
-- @description: Removes all brightness entries
-- @is_runnable: true

DELETE FROM image_brightness;

"#;