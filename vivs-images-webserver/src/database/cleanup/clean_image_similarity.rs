
pub const CLEAN_IMAGE_SIMILARITY_SQL: &str = r#"
-- @name: CLEAN_IMAGE_SIMILARITY_SQL
-- @label: Clean Image Similarity Table
-- @description: Removes similarity entries
-- @is_runnable: true

DELETE FROM image_similarity;

"#;