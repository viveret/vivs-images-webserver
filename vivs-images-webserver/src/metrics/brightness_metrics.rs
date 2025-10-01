use sqlx::SqlitePool;

use crate::actions::analysis_task_item_processor::LogProgListenerPair;
use crate::calc::file_paths_comparison::FilePathComparisonModel;
use crate::filesystem::query::images::get_images_in_photo_sync_path;
use crate::database::query::query_image_brightness::get_image_paths_from_db;


pub async fn get_image_path_comparison_brightness_table_analysis(pool: &SqlitePool, log_prog_listener: Option<LogProgListenerPair>) -> actix_web::Result<FilePathComparisonModel> {
    let image_paths_on_disk = get_images_in_photo_sync_path()?;
    let image_paths_in_sql = get_image_paths_from_db(pool).await?;
    Ok(FilePathComparisonModel::new(
        image_paths_on_disk, "images on disk",
        image_paths_in_sql, "brightness sql list",
        log_prog_listener
    ))
}

pub async fn get_brightness_missing_in_sql_count(pool: &SqlitePool) -> actix_web::Result<(usize, String)> {
    let analysis = get_image_path_comparison_brightness_table_analysis(pool, None).await?;
    let v = analysis.files_missing_from_a.len();
    Ok((v, format!("There are {} images on disk without a known brightness", v)))
}

pub async fn get_brightness_missing_on_disk_count(pool: &SqlitePool) -> actix_web::Result<(usize, String)> {
    let analysis = get_image_path_comparison_brightness_table_analysis(pool, None).await?;
    let v = analysis.files_missing_from_b.len();
    Ok((v, format!("There are {} images in brightness table without a valid image on disk", v)))
}