use std::error::Error;

use sqlx::SqlitePool;

use crate::models::top_level_metrics::TopLevelMetrics;
use crate::database::common::execute_query;


pub async fn query_top_level_metrics(
    pool: &SqlitePool
) -> Result<Vec<sqlx::sqlite::SqliteRow>, Box<dyn Error + Send>> {

    let mut query = String::from("");
    let params: Vec<&str> = vec![];

    query.push_str(r#"
        SELECT COUNT(*) AS total_images FROM image_paths;
        SELECT COUNT(*) AS total_exif FROM image_exif;
        SELECT COUNT(*) AS total_similarity FROM image_similarity;
        SELECT COUNT(*) AS total_brightness FROM image_brightness;
        SELECT COUNT(*) AS total_thumbnails FROM image_thumbnail;
        SELECT COUNT(*) AS total_tags FROM image_tags;
        SELECT COUNT(*) AS total_ocr_text FROM image_ocr_text;
        SELECT COUNT(*) AS total_iptc FROM image_iptc;
    "#);

    let results = execute_query(pool, &query, params).await?;

    Ok(results)
}

pub async fn get_top_level_metrics(pool: &SqlitePool) -> Result<TopLevelMetrics, Box<dyn Error + Send>> {
    Ok(TopLevelMetrics::new(query_top_level_metrics(pool).await?))
}