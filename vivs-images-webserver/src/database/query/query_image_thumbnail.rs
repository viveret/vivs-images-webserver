use std::collections::HashSet;
use std::error::Error;

use sqlx::{Row, SqlitePool};

use crate::models::image_thumbnail::ImageThumbnail;
use crate::database::common::execute_query;



pub async fn get_thumbnail_image_paths_from_db(pool: &SqlitePool) -> Result<HashSet<String>, Box<dyn Error + Send>> {
    let sql = r#"SELECT DISTINCT image_path FROM image_thumbnail"#;
    let rows = execute_query(pool, sql, vec![]).await?;
    
    Ok(rows.iter()
        .filter_map(|r| r.try_get("image_path").ok())
        .collect())
}

pub async fn query_thumbnail_table(image_path: &str, pool: &SqlitePool) -> Result<Vec<ImageThumbnail>, Box<dyn Error + Send>> {
    let sql = r#"SELECT * FROM image_thumbnail WHERE image_path = ?"#;
    let rows = execute_query(pool, sql, vec![ image_path ]).await?;
    let mut items = vec![];
    for r in rows {
        items.push(ImageThumbnail::new_from_row(&r).map_err(|e| Box::new(e) as Box<dyn Error + Send>)?);
    }
    Ok(items)
}

pub async fn query_thumbnail_table_count(image_path: &str, pool: &SqlitePool) -> Result<usize, Box<dyn Error + Send>> {
    let sql = r#"SELECT COUNT(*) 'ct' FROM image_thumbnail WHERE image_path = ?"#;
    let rows = execute_query(pool, sql, vec![ image_path ]).await?;
    let v: Option<u32> = rows.iter().nth(0).map(|r| r.get("ct"));
    let v: usize = v.unwrap_or_default() as usize;
    Ok(v)
}

pub async fn query_thumbnail_table_width_length_operator(
    image_path: &str,
    operator: &str,
    dim: u32,
    order_by: Option<&str>,
    limit: Option<usize>,
    pool: &SqlitePool
) -> Result<Option<ImageThumbnail>, Box<dyn Error + Send>> {
    let order_by = if let Some(order_by) = order_by {
        format!(" ORDER BY {}", order_by)
    } else {
        String::default()
    };
    let limit = if let Some(limit) = limit {
        format!(" LIMIT {}", limit)
    } else {
        String::default()
    };

    let sql = format!(r#"SELECT * FROM image_thumbnail WHERE image_path = ? AND width_and_length {} ?{}{}"#, operator, order_by, limit);
    let dim = dim.to_string();
    let rows = execute_query(pool, &sql, vec![ image_path, &dim ]).await?;
    Ok(if let Some(r) = rows.iter().nth(0) {
        Some(ImageThumbnail::new_from_row(r).map_err(|e| Box::new(e) as Box<dyn Error + Send>)?)
    } else {
        None
    })
}

pub async fn query_thumbnail_table_width_length(image_path: &str, dim: u32, pool: &SqlitePool) -> Result<Option<ImageThumbnail>, Box<dyn Error + Send>> {
    query_thumbnail_table_width_length_operator(image_path, "=", dim, None, None, pool).await
}

pub async fn query_thumbnail_table_at_most_width_length(image_path: &str, dim: u32, pool: &SqlitePool) -> Result<Option<ImageThumbnail>, Box<dyn Error + Send>> {
    query_thumbnail_table_width_length_operator(image_path, "<=", dim, Some("width_and_length DESC"), Some(1), pool).await
}