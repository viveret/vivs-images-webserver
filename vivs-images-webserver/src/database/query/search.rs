use std::collections::HashMap;

use sqlx::SqlitePool;

use crate::models::image::Image;
use crate::database::common::execute_query;
use crate::models::query_params::search_params::SearchParams;


// Modular search function that can be used by different endpoints
pub async fn query_sql_db_images_by_criteria(
    pool: &SqlitePool,
    criteria: &Vec<HashMap<String, String>>,
    order_by: Option<&str>,
    limit: Option<i32>,
    offset: Option<i32>
) -> Result<Vec<sqlx::sqlite::SqliteRow>, actix_web::Error> {
    let mut query = String::from("SELECT image_exif.*, image_brightness.brightness FROM image_exif");
    let mut params: Vec<&str> = Vec::new();
    
    // join on other tables as needed
    query.push_str(" JOIN image_brightness ON image_exif.image_path = image_brightness.image_path");

    let mut query_criteria_sql = String::new();
    for field_group in criteria {
        if !field_group.is_empty() {
            query_criteria_sql.push_str(" AND (");
            let mut inner_sql = String::new();
            for (field, value) in field_group {
                if !value.is_empty() {
                    inner_sql.push_str(&format!(" OR {}", field));
                    params.push(value);
                }
            }
            // Remove trailing " OR "
            inner_sql = inner_sql[4..].to_string();
            query_criteria_sql.push_str(&inner_sql);
            query_criteria_sql.push_str(")");
        }
    }

    if !query_criteria_sql.is_empty() {
        query.push_str(" WHERE ");
        // Remove leading " AND "
        let criteria_sql = &query_criteria_sql[5..];
        query.push_str(criteria_sql);
    }

    if let Some(order) = order_by {
        query.push_str(&format!(" ORDER BY {}", order));
    } else {
        query.push_str(" ORDER BY image_taken_at DESC");
    }

    if let Some(lim) = limit {
        query.push_str(&format!(" LIMIT {}", lim));
    }

    if let Some(offset) = offset {
        query.push_str(&format!(" OFFSET {}", offset));
    }

    execute_query(pool, &query, params).await
}

pub async fn execute_search_images_query_with_criteria(
    pool: &SqlitePool,
    criteria: &Vec<HashMap<String, String>>,
    order_by: Option<&str>,
    limit: Option<i32>,
    offset: Option<i32>
) -> Result<Vec<Image>, actix_web::Error> {
    let results = query_sql_db_images_by_criteria(pool, criteria, order_by, limit, offset).await?;
    Ok(
        results.into_iter()
            .map(|r| Image::new(&r))
            .collect()
    )
}

pub async fn search_images_by_criteria(
    pool: &SqlitePool,
    params: &SearchParams,
    order_by: Option<&str>
) -> Result<SearchImagesPageModel, actix_web::Error> {
    let criteria = params.into_sql_query_params();
    let total_count = execute_search_images_query_with_criteria(pool, &criteria, None, None, None)
        .await?.len();
    let items = execute_search_images_query_with_criteria(pool, &criteria, order_by, params.get_limit(), params.get_offset())
        .await?;
    Ok(SearchImagesPageModel { total_count, items })
}

pub struct SearchImagesPageModel {
    pub total_count: usize,
    pub items: Vec<Image>,
}