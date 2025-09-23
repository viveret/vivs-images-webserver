use sqlx::SqlitePool;

use crate::models::top_level_metrics::TopLevelMetrics;
use crate::database::common::execute_query;


pub async fn query_top_level_metrics(
    pool: &SqlitePool
) -> Result<Vec<sqlx::sqlite::SqliteRow>, actix_web::Error> {

    let mut query = String::from("");
    let params: Vec<&str> = vec![];

    query.push_str(r#"
        SELECT COUNT(*) AS total_images FROM image_exif;
        SELECT COUNT(*) AS total_similarity FROM image_similarity;
        SELECT COUNT(*) AS total_brightness FROM image_brightness;
        SELECT 0 AS total_categories;
    "#);

    let results = execute_query(pool, &query, params).await?;

    Ok(results)
}

pub async fn get_top_level_metrics(pool: &SqlitePool) -> Result<TopLevelMetrics, actix_web::Error> {
    Ok(TopLevelMetrics::new(query_top_level_metrics(pool).await?))
}