use std::collections::{HashMap, HashSet};

use sqlx::{Row, SqlitePool};

use crate::models::image_similarity::ImageComparisonAlgorithm;
use crate::database::common::execute_query;
use crate::converters::extract_image_similarity::compute_comparison_key;



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
            path_a, path_b, path_a, path_b, &hash
        ]
    ).await?;

    let ct: u32 = rows.into_iter().filter_map(|row| row.try_get("ct").ok()).nth(0).unwrap();
    Ok(ct > 0)
}

// Retrieves unique image paths from database
pub async fn get_image_paths_from_db(pool: &SqlitePool) -> actix_web::Result<HashSet<String>> {
    let sql = r#"SELECT image_path_a 'image_path' FROM image_similarity UNION SELECT image_path_b 'image_path' FROM image_similarity"#;
    let rows = execute_query(pool, sql, vec![]).await?;
    
    Ok(rows.iter()
        .filter_map(|r| r.try_get("image_path").ok())
        .collect())
}

pub async fn get_image_path_pairs_from_db(pool: &SqlitePool) -> actix_web::Result<Vec<(String, String)>> {
    let sql = r#"SELECT image_path_a, image_path_b FROM image_similarity;"#;
    let rows = execute_query(pool, sql, vec![]).await?;
    
    let v: Vec<(String, String)> = rows.iter()
        .filter_map(|r| {
            if let Some(a) = r.try_get("image_path_a").ok() {
                if let Some(b) = r.try_get("image_path_b").ok() {
                    Some((a, b))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    Ok(v)
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

pub async fn query_similarity_table_count(path: &str, pool: &SqlitePool) -> actix_web::Result<usize> {
    let sql = r#"SELECT COUNT(*) 'ct' FROM image_similarity WHERE image_path_a = ? OR image_path_b = ?;"#;
    let rows = execute_query(pool, sql, vec![ path, path ]).await?;

    let v: u32 = rows.iter()
        .filter_map(|r| r.try_get("ct").ok())
        .nth(0)
        .unwrap_or_default();
    Ok(v as usize)
}

pub async fn query_similarity_table_paths_using_thumbnail_algo(comp_algo: ImageComparisonAlgorithm, pool: &SqlitePool) -> actix_web::Result<Vec<String>> {
    let sql = r#"
    SELECT DISTINCT image_path_a 'path' FROM image_similarity WHERE image_comparison_algorithm = ? UNION
        SELECT DISTINCT image_path_b 'path' FROM image_similarity WHERE image_comparison_algorithm = ?
    ;"#;
    let comp_algo = (comp_algo as u8).to_string();
    let rows = execute_query(pool, sql, vec![ &comp_algo, &comp_algo ]).await?;

    let v: Vec<String> = rows.iter()
        .filter_map(|r| r.try_get("path").ok())
        .collect();
    Ok(v)
}

pub async fn query_similarity_table_pairs_using_thumbnail_algo(comp_algo: ImageComparisonAlgorithm, pool: &SqlitePool) -> actix_web::Result<Vec<(String, String)>> {
    let sql = r#"
    SELECT image_path_a, image_path_b FROM image_similarity WHERE image_comparison_algorithm = ?;"#;
    let comp_algo = (comp_algo as u8).to_string();
    let rows = execute_query(pool, sql, vec![ &comp_algo ]).await?;

    let v: Vec<(String, String)> = rows.iter()
        .filter_map(|r| {
            if let Some(a) = r.try_get("image_path_a").ok() {
                if let Some(b) = r.try_get("image_path_b").ok() {
                    Some((a, b))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    Ok(v)
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