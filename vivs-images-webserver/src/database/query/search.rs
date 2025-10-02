use std::collections::HashMap;

use sqlx::{Row, SqlitePool};

use crate::database::query::query_image_thumbnail::query_thumbnail_table_at_most_width_length;
use crate::models::image::Image;
use crate::database::common::execute_query;
use crate::models::query_params::search_params::SearchParams;

// Core query executor that can be customized with select clause and row handler
pub async fn execute_custom_image_query<F, T>(
    pool: &SqlitePool,
    criteria: &Vec<(String, HashMap<String, String>)>,
    order_by: Option<&str>,
    limit: Option<i32>,
    offset: Option<i32>,
    select_clause: &str,
    row_handler: F,
) -> Result<Vec<T>, actix_web::Error>
where
    F: Fn(sqlx::sqlite::SqliteRow) -> T,
{
    let mut query = String::from(select_clause);
    let mut params: Vec<&str> = Vec::new();
    
    // join on other tables as needed
    query.push_str(" LEFT JOIN image_brightness ON image_exif.image_path = image_brightness.image_path");
    query.push_str(" LEFT JOIN image_ocr_text ON image_exif.image_path = image_ocr_text.image_path");
    query.push_str(" LEFT JOIN image_aspect_ratio ON image_exif.image_path = image_aspect_ratio.image_path");

    let mut query_criteria_sql = String::new();
    for (i, (field_op, field_group)) in criteria.iter().enumerate() {
        if !field_group.is_empty() {
            query_criteria_sql.push_str(" AND (");
            let mut inner_sql = String::new();
            for (field, value) in field_group {
                if !value.is_empty() {
                    inner_sql.push_str(&format!(" {} {}", field_op, field));
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

    let results = execute_query(pool, &query, params).await?;
    Ok(results.into_iter().map(row_handler).collect())
}

pub async fn query_sql_db_images_by_criteria(
    pool: &SqlitePool,
    criteria: &Vec<(String, HashMap<String, String>)>,
    order_by: Option<&str>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<sqlx::sqlite::SqliteRow>, actix_web::Error> {
    let select_clause = r#"
    SELECT image_exif.*,
    image_brightness.brightness,
    image_ocr_text.ocr_text,
    image_aspect_ratio.width_pixels,
    image_aspect_ratio.height_pixels,
    image_aspect_ratio.aspect_ratio,
    image_aspect_ratio.quality
    FROM image_exif
    "#;
    
    execute_custom_image_query(
        pool, 
        criteria, 
        order_by, 
        limit, 
        offset, 
        select_clause, 
        |row| row
    ).await
}

// get count without fetching full image data
pub async fn count_sql_db_images_by_criteria(
    pool: &SqlitePool,
    criteria: &Vec<(String, HashMap<String, String>)>,
) -> Result<i64, actix_web::Error> {
    let select_clause = "SELECT COUNT(*) as count FROM image_exif";
    
    let results = execute_custom_image_query(
        pool, 
        criteria, 
        None, 
        None, 
        None, 
        select_clause, 
        |row| row
    ).await?;
    
    if let Some(row) = results.first() {
        let count: i64 = row.try_get("count").unwrap_or(0);
        Ok(count)
    } else {
        Ok(0)
    }
}

pub async fn execute_search_images_query_with_criteria(
    pool: &SqlitePool,
    criteria: &Vec<(String, HashMap<String, String>)>,
    order_by: Option<&str>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<Image>, actix_web::Error> {
    let results = query_sql_db_images_by_criteria(pool, criteria, order_by, limit, offset).await?;
    let mut results: Vec<Image> = results.into_iter()
            .map(|r| Image::new(&r))
            .collect();

    for img in results.iter_mut() {
        if let Some(thumb) = query_thumbnail_table_at_most_width_length(&img.path, 64, pool).await? {
            img.assign_thumbnail(thumb);
        }
    }
    Ok(results)
}

pub async fn search_images_by_criteria(
    pool: &SqlitePool,
    params: &SearchParams,
    order_by: Option<&str>,
) -> Result<SearchImagesPageModel, actix_web::Error> {
    let criteria = params.into_sql_query_params();
    
    // Use the new count function instead of fetching all data
    let total_count = count_sql_db_images_by_criteria(pool, &criteria).await? as usize;
    
    let items = execute_search_images_query_with_criteria(pool, &criteria, order_by, params.get_limit(), params.get_offset())
        .await?;
    
    println!("returning {} of {} search results (asked for {:?})", items.len(), total_count, params.get_limit());
    Ok(SearchImagesPageModel { total_count, items })
}

pub struct SearchImagesPageModel {
    pub total_count: usize,
    pub items: Vec<Image>,
}

pub async fn find_image_by_path(pool: &SqlitePool, path: &str) -> Result<Option<Image>, actix_web::Error> {
    let mut params = HashMap::new();
    params.insert("image_exif.image_path = ?".to_string(), path.to_string());
    let criteria = vec![ ("".to_string(), params) ];
    let results = execute_search_images_query_with_criteria(pool, &criteria, None, Some(1), None).await?;
    if let Some(item) = results.first() {
        Ok(Some(item.clone()))
    } else {
        Ok(None)
    }
}