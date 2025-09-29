use sqlx::SqlitePool;

use crate::calc::file_paths_comparison::FilePathComparisonModel;
use crate::database::query::query_image_similarity::get_image_paths_from_db;
use crate::filesystem::query::images::get_images_in_photo_sync_path;


// could make this more generic and put into a path comparison analysis class
pub async fn get_image_path_comparison_analysis(pool: &SqlitePool) -> actix_web::Result<FilePathComparisonModel> {
    let image_paths_on_disk = get_images_in_photo_sync_path()?;
    let image_paths_in_sql = get_image_paths_from_db(pool).await?;
    Ok(FilePathComparisonModel::new(
        image_paths_on_disk, "images on disk",
        image_paths_in_sql, "similarity sql list")
    )
}

pub async fn get_similarity_missing_in_sql_count(pool: &SqlitePool) -> actix_web::Result<(usize, String)> {
    let analysis = get_image_path_comparison_analysis(pool).await?;
    let v = analysis.files_missing_from_a.len();
    Ok((v, format!("There are {} images on disk without a known similarity", v)))
}

pub async fn get_similarity_missing_on_disk_count(pool: &SqlitePool) -> actix_web::Result<(usize, String)> {
    let analysis = get_image_path_comparison_analysis(pool).await?;
    let v = analysis.files_missing_from_b.len();
    Ok((v, format!("There are {} images in SQL without a valid image on disk", v)))
}
