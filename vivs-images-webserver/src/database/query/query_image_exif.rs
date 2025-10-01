use std::collections::HashSet;

use sqlx::{Row, SqlitePool};

use crate::database::common::execute_query;



// Retrieves exif image paths from the exif table in the database
pub async fn get_image_paths_from_db(pool: &SqlitePool) -> actix_web::Result<HashSet<String>> {
    let sql = r#"SELECT image_path FROM image_exif"#;
    let rows = execute_query(pool, sql, vec![]).await?;
    
    Ok(rows.iter()
        .filter_map(|r| r.try_get("image_path").ok())
        .collect())
}


pub async fn query_exif_table_count(image_path: &str, pool: &SqlitePool) -> actix_web::Result<usize> {
    let sql = r#"SELECT COUNT(*) 'ct' FROM image_exif WHERE image_path = ?"#;
    let rows = execute_query(pool, sql, vec![ image_path ]).await?;
    let v: Option<u32> = rows.iter().nth(0).map(|r| r.get("ct"));
    let v: usize = v.unwrap_or_default() as usize;
    Ok(v)
}