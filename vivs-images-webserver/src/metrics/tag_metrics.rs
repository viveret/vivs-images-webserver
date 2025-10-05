use std::error::Error;

use sqlx::SqlitePool;

use crate::actions::analysis_task_item_processor::LogProgListenerPair;
use crate::database::query::query_image_tag::get_image_paths_from_tags_in_sql_db;
use crate::calc::file_paths_comparison::FilePathComparisonModel;
use crate::filesystem::query::images::get_images_in_photo_sync_path;



pub async fn get_image_path_comparison_tags_table_files_with_tags_tags_analysis(log_prog_listener: Option<LogProgListenerPair>, pool: &SqlitePool) -> Result<FilePathComparisonModel, Box<dyn Error + Send>> {
    let image_paths_on_disk = get_images_in_photo_sync_path()?;
    let image_paths_in_sql = get_image_paths_from_tags_in_sql_db(pool).await?;
    Ok(FilePathComparisonModel::new(
        image_paths_on_disk, "images on disk",
        image_paths_in_sql, "image tag sql list",
        log_prog_listener
    ))
}

pub async fn get_tags_missing_in_sql_count(pool: &SqlitePool) -> Result<(usize, String), Box<dyn Error + Send>> {
    let analysis = get_image_path_comparison_tags_table_files_with_tags_tags_analysis(None, pool).await?;
    let v = analysis.files_missing_from_b.len();
    Ok((v, format!("There are {} images on disk without a known tags entry", v)))
}

pub async fn get_tags_missing_on_disk_count(pool: &SqlitePool) -> Result<(usize, String), Box<dyn Error + Send>> {
    let analysis = get_image_path_comparison_tags_table_files_with_tags_tags_analysis(None, pool).await?;
    let v = analysis.files_missing_from_a.len();
    Ok((v, format!("There are {} images in the tags SQL table without a valid image on disk", v)))
}