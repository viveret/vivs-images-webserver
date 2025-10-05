use std::error::Error;

use sqlx::{Pool, Sqlite};

use crate::database::common::execute_update_or_insert;


pub async fn execute_update_image_brightness_sql(image_path: &String, brightness: f32, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error + Send>> {
    let query = r#"
        UPDATE image_brightness
        SET brightness = ?, updated_at = CURRENT_TIMESTAMP
        WHERE image_path = ?;
    "#;
    let r = execute_update_or_insert(pool, query, vec![ brightness.to_string().as_str(), image_path ]).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL update returned {} rows", r))))
    }
}

pub async fn execute_insert_image_brightness_sql(image_path: &String, brightness: f32, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error + Send>> {
    let query = r#"INSERT INTO image_brightness (image_path, brightness) VALUES (?, ?);"#;
    let r = execute_update_or_insert(pool, query, vec![ image_path, brightness.to_string().as_str() ]).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL insert returned {} rows", r))))
    }
}

pub async fn execute_delete_image_brightness_sql(image_path: &String, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error + Send>> {
    let query = r#"DELETE FROM image_brightness WHERE image_path = ?;"#;
    let r = execute_update_or_insert(pool, query, vec![ image_path ]).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL delete returned {} rows", r))))
    }
}