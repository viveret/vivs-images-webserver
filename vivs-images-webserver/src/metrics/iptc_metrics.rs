use std::error::Error;

use sqlx::SqlitePool;

use crate::calc::file_paths_comparison::FilePathComparisonModel;
use crate::filesystem::query::images::get_jpg_tiff_in_photo_sync_path;
use crate::database::query::query_image_iptc::get_image_paths_from_iptc_sql_db;


pub async fn get_image_path_comparison_iptc_table_analysis(pool: &SqlitePool) -> Result<FilePathComparisonModel, Box<dyn Error + Send>> {
    let image_paths_on_disk = get_jpg_tiff_in_photo_sync_path()?;
    let image_paths_in_sql = get_image_paths_from_iptc_sql_db(pool).await?;
    Ok(FilePathComparisonModel::new(
        image_paths_on_disk, "jpeg and tiff images on disk",
        image_paths_in_sql, "iptc sql list",
        None
    ))
}

pub async fn get_iptc_missing_in_sql_count(pool: &SqlitePool) -> Result<(usize, String), Box<dyn Error + Send>> {
    let analysis = get_image_path_comparison_iptc_table_analysis(pool).await?;
    let v = analysis.files_missing_from_b.len();
    Ok((v, format!("There are {} images on disk without a known iptc entry", v)))
}

pub async fn get_iptc_missing_on_disk_count(pool: &SqlitePool) -> Result<(usize, String), Box<dyn Error + Send>> {
    let analysis = get_image_path_comparison_iptc_table_analysis(pool).await?;
    let v = analysis.files_missing_from_a.len();
    Ok((v, format!("There are {} images in the iptc SQL table without a valid image on disk", v)))
}