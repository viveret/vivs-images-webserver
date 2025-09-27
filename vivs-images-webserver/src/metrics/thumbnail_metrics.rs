use sqlx::SqlitePool;

use crate::converters::comparison::compare_paths;
use crate::database::query::query_image_thumbnail::get_thumbnail_image_paths_from_db;
use crate::filesystem::query::images::get_images_in_photo_sync_path;


#[derive(Clone)]
pub struct ThumbnailMissingAnalysis {
    pub total_differences: usize,
    pub files_missing_from_sql: Vec<String>,
    pub files_missing_from_disk: Vec<String>,
    pub message: String,
    pub log: String,
    pub log_error: String,
}

pub async fn get_image_path_comparison_thumbnail_table_analysis(pool: &SqlitePool) -> actix_web::Result<ThumbnailMissingAnalysis> {
    let image_paths_on_disk = get_images_in_photo_sync_path()?;
    let image_paths_in_sql = get_thumbnail_image_paths_from_db(pool).await?;
    let mut log = String::new();
    let mut log_error = String::new();
    
    log.push_str(&format!("Comparing {} images on disk to {} in thumbnail sql list", 
        image_paths_on_disk.len(), 
        image_paths_in_sql.len()
    ));
    
    let (files_missing_from_sql, files_missing_from_disk, total_differences) = 
        compare_paths(&image_paths_on_disk, &image_paths_in_sql);
    
    Ok(
        ThumbnailMissingAnalysis {
        total_differences,
        files_missing_from_sql,
        files_missing_from_disk,
        message: format!("There are {} thumbnail file differences", total_differences),
        log, log_error
    })
}

pub async fn get_thumbnail_missing_in_sql_count(pool: &SqlitePool) -> actix_web::Result<(usize, String)> {
    let analysis = get_image_path_comparison_thumbnail_table_analysis(pool).await?;
    let v = analysis.files_missing_from_sql.len();
    Ok((v, format!("There are {} images on disk without a known thumbnail", v)))
}

pub async fn get_thumbnail_missing_on_disk_count(pool: &SqlitePool) -> actix_web::Result<(usize, String)> {
    let analysis = get_image_path_comparison_thumbnail_table_analysis(pool).await?;
    let v = analysis.files_missing_from_disk.len();
    Ok((v, format!("There are {} thumbnail images in SQL without a valid image on disk", v)))
}
