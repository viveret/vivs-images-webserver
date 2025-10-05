use std::error::Error;

use sqlx::{Pool, Sqlite};

use crate::models::image_aspect_ratio::ImageAspectRatio;
use crate::database::common::execute_update_or_insert;


pub async fn execute_update_image_aspect_ratio_sql(image_path: &String, aspect_ratio: f32, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error + Send>> {
    let query = r#"
        UPDATE image_aspect_ratio
        SET aspect_ratio = ?
        WHERE image_path = ?;
    "#;
    let r = execute_update_or_insert(pool, query, vec![ aspect_ratio.to_string().as_str(), image_path ]).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL update returned {} rows", r))))
    }
}

pub async fn execute_insert_image_aspect_ratio_sql(item: &ImageAspectRatio, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error + Send>> {
    let column_names = ImageAspectRatio::get_meta().iter().map(|c| c.name.to_string()).collect::<Vec<String>>();
    let column_names_sql = column_names.join(", ");
    let column_var_placeholders_sql = column_names.iter().map(|_| "?").collect::<Vec<&str>>().join(", ");
    let query = format!(r#"INSERT INTO image_aspect_ratio ({}) VALUES ({});"#, column_names_sql, column_var_placeholders_sql);
    let params: Vec<String> = column_names.iter().map(|c| item.get_field(c).unwrap()).collect();
    let params: Vec<&str> = params.iter().map(|c| c.as_str()).collect();
    let r = execute_update_or_insert(&pool, &query, params).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL insert returned {} rows", r))))
    }
}

pub async fn execute_delete_image_aspect_ratio_sql(image_path: &String, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error + Send>> {
    let query = r#"DELETE FROM image_aspect_ratio WHERE image_path = ?;"#;
    let r = execute_update_or_insert(pool, query, vec![ image_path ]).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL delete returned {} rows", r))))
    }
}