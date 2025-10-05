use std::collections::HashSet;
use std::error::Error;

use sqlx::{Row, SqlitePool};

use crate::models::image_ocr_text::ImageOcrText;
use crate::filesystem::query::images::get_photo_sync_path;
use crate::filesystem::query::images::get_image_ocr_text_export_path;
use crate::filesystem::query::images::change_base_path_of_paths;
use crate::database::common::execute_query;


// Retrieves image paths from the ocr_text table in the database
pub async fn get_ocr_text_image_paths_from_db(pool: &SqlitePool) -> Result<HashSet<String>, Box<dyn Error + Send>> {
    let sql = r#"SELECT image_path FROM image_ocr_text"#;
    let rows = execute_query(pool, sql, vec![]).await?;
    
    Ok(rows.iter()
        .filter_map(|r| r.try_get("image_path").ok())
        .collect())
}

pub async fn query_ocr_text_from_db(image_path: &str, pool: &SqlitePool) -> Result<Option<ImageOcrText>, Box<dyn Error + Send>> {
    let sql = r#"SELECT image_path, ocr_text FROM image_ocr_text WHERE image_path = ?"#;
    let rows = execute_query(pool, sql, vec![ image_path ]).await?;
    let v: Option<ImageOcrText> = rows.iter().map(ImageOcrText::new).nth(0);
    Ok(v)
}

pub async fn query_ocr_text_table_count(image_path: &str, pool: &SqlitePool) -> Result<usize, Box<dyn Error + Send>> {
    let sql = r#"SELECT COUNT(*) 'ct' FROM image_ocr_text WHERE image_path = ?"#;
    let rows = execute_query(pool, sql, vec![ image_path ]).await?;
    let v: Option<u32> = rows.iter().nth(0).map(|r| r.get("ct"));
    let v: usize = v.unwrap_or_default() as usize;
    Ok(v)
}

pub async fn get_expected_ocr_text_file_paths_from_db(pool: &SqlitePool) -> Result<HashSet<String>, Box<dyn Error + Send>> {
    let image_paths = get_ocr_text_image_paths_from_db(pool).await?;
    let new_base_path = get_image_ocr_text_export_path()?;
    let old_base_path = get_photo_sync_path()?;
    change_base_path_of_paths(image_paths, old_base_path, new_base_path)
        .map_err(|e| Box::new(e) as Box<dyn Error + Send>)
}
