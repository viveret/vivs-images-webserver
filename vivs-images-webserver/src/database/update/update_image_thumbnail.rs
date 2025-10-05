use std::error::Error;

use actix_web::Either;
use sqlx::{Pool, Sqlite};

use crate::models::image_thumbnail::ImageThumbnail;
use crate::database::common::{execute_update_or_insert, execute_update_or_insert_with_blob};


pub async fn execute_update_image_thumbnail_sql(image_path: &String, thumbnail: f64, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error + Send>> {
    let query = r#"
        UPDATE image_thumbnail
        SET thumbnail = ?, updated_at = CURRENT_TIMESTAMP
        WHERE image_path = ?;
    "#;
    let r = execute_update_or_insert(pool, query, vec![ thumbnail.to_string().as_str(), image_path ]).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL update returned {} rows", r))))
    }
}

pub async fn execute_insert_image_thumbnail_sql(thumbnail: &ImageThumbnail, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error + Send>> {
    let regular_column_names = ImageThumbnail::get_meta().iter().filter(|c| c.field_type != "blob").map(|c| c.name.to_string()).collect::<Vec<String>>();
    let blob_column_names = ImageThumbnail::get_meta().iter().filter(|c| c.field_type == "blob").map(|c| c.name.to_string()).collect::<Vec<String>>();
    let column_names_sql = regular_column_names.join(", ");
    let blob_column_names_sql = blob_column_names.join(", ");
    let column_var_placeholders_sql = std::iter::repeat_n("?", ImageThumbnail::get_meta().len()).collect::<Vec<&str>>().join(", ");
    let query = format!(r#"INSERT INTO image_thumbnail ({},{}) VALUES ({});"#, column_names_sql, blob_column_names_sql, column_var_placeholders_sql);
    let params: Vec<String> = regular_column_names.iter().filter_map(|c| thumbnail.get_field(c)).collect();
    let mut params: Vec<Either<&str, Vec<u8>>> = params.iter().map(|v| Either::Left(v.as_str())).collect();
    for blob_c in blob_column_names.iter() {
        if let Some(blob) = thumbnail.get_field_blob(blob_c) {
            params.push(Either::Right(blob));
        } else {
            return Err(Box::new(std::io::Error::other(format!("Could not get blob from data object"))));
        }
    }
    let r = execute_update_or_insert_with_blob(&pool, &query, params).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL insert returned {} rows", r))))
    }
}

pub async fn execute_delete_image_thumbnail_sql(image_path: &String, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error + Send>> {
    let query = r#"DELETE FROM image_thumbnail WHERE image_path = ?;"#;
    let r = execute_update_or_insert(pool, query, vec![ image_path ]).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL delete returned {} rows", r))))
    }
}