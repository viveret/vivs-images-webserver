use sqlx::SqlitePool;

use crate::actions::analysis_task_item_processor::LogProgListenerPair;
use crate::database::query::query_image_ocr_text::{get_expected_ocr_text_file_paths_from_db, get_ocr_text_image_paths_from_db};
use crate::filesystem::query::images::{get_images_in_photo_sync_path, get_ocr_text_file_paths_in_doc_sync_path};
use crate::calc::file_paths_comparison::FilePathComparisonModel;



pub async fn get_image_path_comparison_ocr_text_table_analysis(
    pool: &SqlitePool, log_prog_listener: Option<LogProgListenerPair>
) -> actix_web::Result<FilePathComparisonModel> {
    let image_paths_on_disk = get_images_in_photo_sync_path()?;
    let image_paths_in_sql = get_ocr_text_image_paths_from_db(pool).await?;
    Ok(FilePathComparisonModel::new(
        image_paths_on_disk, "images on disk",
        image_paths_in_sql, "ocr_text sql list",
        log_prog_listener
    ))
}

pub async fn get_ocr_text_missing_in_sql_count(pool: &SqlitePool) -> actix_web::Result<(usize, String)> {
    let analysis = get_image_path_comparison_ocr_text_table_analysis(pool, None).await?;
    let v = analysis.files_missing_from_b.len();
    Ok((v, format!("There are {} images on disk without a known ocr_text entry", v)))
}

pub async fn get_ocr_text_missing_on_disk_count(pool: &SqlitePool) -> actix_web::Result<(usize, String)> {
    let analysis = get_image_path_comparison_ocr_text_table_analysis(pool, None).await?;
    let v = analysis.files_missing_from_a.len();
    Ok((v, format!("There are {} images in the ocr_text SQL table without a valid image on disk", v)))
}





pub async fn get_ocr_text_file_path_comparison_ocr_text_table_analysis(
    pool: &SqlitePool, log_prog_listener: Option<LogProgListenerPair>
) -> actix_web::Result<FilePathComparisonModel> {
    let expected_paths_from_sql = get_expected_ocr_text_file_paths_from_db(pool).await?;
    let ocr_text_file_paths_on_disk = get_ocr_text_file_paths_in_doc_sync_path()?;
    Ok(FilePathComparisonModel::new(
        expected_paths_from_sql, "expected ocr text files from sql",
        ocr_text_file_paths_on_disk, "ocr text files on disk",
        log_prog_listener
    ))
}