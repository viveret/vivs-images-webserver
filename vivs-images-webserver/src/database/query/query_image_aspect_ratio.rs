use std::error::Error;
use std::collections::HashSet;

use sqlx::{Row, SqlitePool};

use crate::database::common::execute_query;


// Retrieves aspect ratio image paths from the aspect_ratio table in the database
pub async fn get_image_paths_from_db(pool: &SqlitePool) -> Result<HashSet<String>, Box<dyn Error + Send>> {
    let sql = r#"SELECT image_path FROM image_aspect_ratio"#;
    let rows = execute_query(pool, sql, vec![]).await?;
    
    Ok(rows.iter()
        .filter_map(|r| r.try_get("image_path").ok())
        .collect())
}


pub async fn query_aspect_ratio_table_count(image_path: &str, pool: &SqlitePool) -> Result<usize, Box<dyn Error + Send>> {
    let sql = r#"SELECT COUNT(*) 'ct' FROM image_aspect_ratio WHERE image_path = ?"#;
    let rows = execute_query(pool, sql, vec![ image_path ]).await?;
    let v: Option<u32> = rows.iter().nth(0).map(|r| r.get("ct"));
    let v: usize = v.unwrap_or_default() as usize;
    Ok(v)
}