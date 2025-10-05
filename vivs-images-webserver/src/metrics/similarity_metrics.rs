use std::error::Error;

use sqlx::SqlitePool;

use crate::actions::analysis_task_item_processor::LogProgListenerPair;
use crate::calc::file_paths_comparison::{CrossFilePathComparisonModel, FilePathComparisonModel};
use crate::database::query::query_image_similarity::{get_image_path_pairs_from_db, get_image_paths_from_db};
use crate::filesystem::query::images::get_images_in_photo_sync_path;


pub async fn get_image_paths_simple_difference_similarity_analysis(pool: &SqlitePool) -> Result<FilePathComparisonModel, Box<dyn Error + Send>> {
    let image_paths_on_disk = get_images_in_photo_sync_path()?;
    let image_paths_in_sql = get_image_paths_from_db(pool).await?;
    Ok(FilePathComparisonModel::new(
        image_paths_on_disk, "images on disk",
        image_paths_in_sql, "similarity sql list", None)
    )
}

pub async fn get_simple_similarity_missing_in_sql_count(pool: &SqlitePool) -> Result<(usize, String), Box<dyn Error + Send>> {
    let analysis = get_image_paths_simple_difference_similarity_analysis(pool).await?;
    let v = analysis.files_missing_from_a.len();
    Ok((v, format!("There are {} images on disk without a known similarity", v)))
}

pub async fn get_simple_similarity_missing_on_disk_count(pool: &SqlitePool) -> Result<(usize, String), Box<dyn Error + Send>> {
    let analysis = get_image_paths_simple_difference_similarity_analysis(pool).await?;
    let v = analysis.files_missing_from_b.len();
    Ok((v, format!("There are {} images in SQL without a valid image on disk", v)))
}




pub async fn get_image_paths_full_difference_similarity_analysis(
    pool: &SqlitePool, log_prog_listener: Option<LogProgListenerPair>
) -> Result<CrossFilePathComparisonModel, Box<dyn Error + Send>> {
    if let Some(x) = &log_prog_listener {
        x.1("getting image paths in photo sync path");
        x.0(0.3);
    }
    let image_paths_on_disk = get_images_in_photo_sync_path()?;
    
    if let Some(x) = &log_prog_listener {
        x.1("getting image pairs from db");
        x.0(0.6);
    }
    let image_paths_in_sql = get_image_path_pairs_from_db(pool).await?;
    Ok(CrossFilePathComparisonModel::new_easy_2(
        image_paths_on_disk, "images on disk",
        image_paths_in_sql, "similarity sql list",
        log_prog_listener
    ))
}

pub async fn get_full_similarity_missing_in_sql_count(pool: &SqlitePool) -> Result<(usize, String), Box<dyn Error + Send>> {
    let analysis = get_image_paths_full_difference_similarity_analysis(pool, None).await?;
    let v = analysis.pairs_missing_from_a.len();
    Ok((v, format!("There are {} image pairs on disk without a known similarity", v)))
}

pub async fn get_full_similarity_missing_on_disk_count(pool: &SqlitePool) -> Result<(usize, String), Box<dyn Error + Send>> {
    let analysis = get_image_paths_full_difference_similarity_analysis(pool, None).await?;
    let v = analysis.pairs_missing_from_b.len();
    Ok((v, format!("There are {} image pairs in SQL without a valid image on disk", v)))
}
