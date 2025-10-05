use std::error::Error;

use sqlx::SqlitePool;

use crate::{database::common::execute_update_or_insert, models::image_ocr_text::ImageOcrText};



pub async fn execute_insert_image_ocr_text_sql(item: &ImageOcrText, pool: SqlitePool) -> Result<(), Box<dyn Error + Send>> {
    let query = r#"INSERT INTO image_ocr_text (image_path, ocr_text) VALUES (?, ?);"#;
    let r = execute_update_or_insert(&pool, query, vec![ &item.image_path, &item.ocr_text ]).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL insert returned {} rows", r))))
    }
}