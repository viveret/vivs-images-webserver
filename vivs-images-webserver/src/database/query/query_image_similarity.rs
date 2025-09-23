use std::collections::HashMap;

use sqlx::{Row, SqlitePool};

use crate::{converters::extract_image_similarity::compute_comparison_key, database::common::execute_query};



pub async fn get_image_similarity_value_exists_in_db(path_a: &str, path_b: &str, pool: &SqlitePool) -> actix_web::Result<bool> {
    let hash = compute_comparison_key(path_a, path_b).to_string();
    let rows = execute_query(pool,
        r#"
        SELECT COUNT(*) 'ct'
        FROM image_similarity
        WHERE 
            (image_path_a = ? AND image_path_b = ?)
            OR (image_path_b = ? AND image_path_a = ?)
            OR image_comparison_key = ?;
        "#,
        vec![
            path_a, path_b, path_b, path_a, &hash
        ]
    ).await?;

    let ct: u32 = rows.into_iter().filter_map(|row| row.try_get("ct").ok()).nth(0).unwrap();
    Ok(ct > 0)
}

// Retrieves unique image paths from database
pub async fn get_image_paths_from_db(pool: &SqlitePool) -> actix_web::Result<Vec<String>> {
    let sql = r#"SELECT image_path_a 'image_path' FROM image_similarity UNION SELECT image_path_b 'image_path' FROM image_similarity"#;
    let rows = execute_query(pool, sql, vec![]).await?;
    
    Ok(rows.iter()
        .filter_map(|r| r.try_get("image_path").ok())
        .collect())
}

// Retrieves the amount of unique image paths from database
pub async fn get_count_of_image_paths_from_db(pool: &SqlitePool) -> actix_web::Result<u32> {
    let sql = r#"SELECT COUNT(*) 'ct' FROM (SELECT image_path_a 'image_path' FROM image_similarity UNION SELECT image_path_b 'image_path' FROM image_similarity)"#;
    let rows = execute_query(pool, sql, vec![]).await?;
    
    Ok(rows.iter()
        .filter_map(|r| r.try_get("ct").ok())
        .nth(0)
        .unwrap_or_default())
}

pub async fn get_similarity_table_count_from_db(pool: &SqlitePool) -> actix_web::Result<usize> {
    let sql = r#"SELECT COUNT(*) 'ct' FROM image_similarity;"#;
    let rows = execute_query(pool, sql, vec![]).await?;

    let v: u32 = rows.iter()
        .filter_map(|r| r.try_get("ct").ok())
        .nth(0)
        .unwrap_or_default();
    Ok(v as usize)
}

pub async fn get_count_of_comparisons_per_image_path(pool: &SqlitePool) -> actix_web::Result<HashMap<String, u32>> {
    let sql = r#"SELECT image_path_a as image_path, COUNT(*) as ct FROM image_similarity GROUP BY image_path_a;"#;
    let rows = execute_query(pool, sql, vec![]).await?;
    
    Ok(rows.iter()
        .map(|r| (r.try_get("image_path"), r.try_get("ct")))
        .filter_map(|result| {
            match result {
                (Ok(image_path), Ok(count)) => {
                    Some((image_path, count))
                }
                _ => None
            }
        })
        .collect())
}