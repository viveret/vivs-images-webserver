use std::error::Error;

use sqlx::{Pool, Sqlite};

use crate::database::common::execute_update_or_insert;


pub async fn execute_update_image_xmp_sql(image_path: &String, xmp: &String, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error + Send>> {
    let query = r#"
        UPDATE image_xmp
        SET xmp = ?
        WHERE image_path = ?;
    "#;
    let r = execute_update_or_insert(pool, query, vec![ xmp, image_path ]).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL update returned {} rows", r))))
    }
}

pub async fn execute_insert_image_xmp_sql(image_path: &String, xmp: &String, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error + Send>> {
    let query = r#"INSERT INTO image_xmp (image_path, xmp) VALUES (?, ?);"#;
    let r = execute_update_or_insert(pool, query, vec![ image_path, xmp ]).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL insert returned {} rows", r))))
    }
}

pub async fn execute_delete_image_xmp_sql(image_path: &String, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error + Send>> {
    let query = r#"DELETE FROM image_xmp WHERE image_path = ?;"#;
    let r = execute_update_or_insert(pool, query, vec![ image_path ]).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL delete returned {} rows", r))))
    }
}