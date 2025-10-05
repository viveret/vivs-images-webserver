use std::{collections::HashSet, error::Error};

use sqlx::{Row, SqlitePool};

use crate::database::common::execute_query;


pub async fn get_image_paths_from_db(pool: &SqlitePool) -> Result<HashSet<String>, Box<dyn Error + Send>> {
    let sql = r#"SELECT image_path FROM image_paths"#;
    let rows = execute_query(pool, sql, vec![]).await?;
    
    Ok(rows.iter()
        .filter_map(|r| r.try_get("image_path").ok())
        .collect())
}


pub async fn query_image_path_table_count(image_path: &str, pool: &SqlitePool) -> Result<usize, Box<dyn Error + Send>> {
    let sql = r#"SELECT COUNT(*) 'ct' FROM image_paths WHERE image_path = ?"#;
    let rows = execute_query(pool, sql, vec![ image_path ]).await?;
    let v: Option<u32> = rows.iter().nth(0).map(|r| r.get("ct"));
    let v: usize = v.unwrap_or_default() as usize;
    Ok(v)
}