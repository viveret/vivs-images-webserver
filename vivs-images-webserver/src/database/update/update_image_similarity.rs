use std::error::Error;

use sqlx::{Pool, Sqlite};

use crate::models::image_similarity::ImageSimilarity;
use crate::database::common::execute_update_or_insert;
use crate::converters::extract_image_similarity::compute_comparison_key;


pub async fn execute_update_image_similarity_sql(v: ImageSimilarity, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error + Send>> {
    let query = r#"
        UPDATE image_similarity
        SET similarity_value = ?
        WHERE image_comparison_key = ?;
    "#;
    let similarity_value = v.similarity_value.to_string();
    let hash = compute_comparison_key(&v.image_path_a, &v.image_path_b).to_string();
    let params: Vec<&str> = vec![ &similarity_value, &hash ];
    let r = execute_update_or_insert(pool, query, params).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL update returned {} rows", r))))
    }
}

pub async fn execute_insert_image_similarity_sql(v: &ImageSimilarity, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error + Send>> {
    let column_names = ImageSimilarity::get_meta().iter().map(|c| c.name.to_string()).collect::<Vec<String>>();
    let column_names_sql = column_names.join(", ");
    let column_var_placeholders_sql = column_names.iter().map(|_| "?").collect::<Vec<&str>>().join(", ");
    let query = format!(r#"INSERT INTO image_similarity ({}) VALUES ({});"#, column_names_sql, column_var_placeholders_sql);
    let params: Vec<String> = column_names.iter().filter_map(|c| v.get_field(c)).collect();
    let params = params.iter().map(|x| x.as_str()).collect();
    let r = execute_update_or_insert(&pool, &query, params).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL insert returned {} rows", r))))
    }
}

pub async fn execute_delete_image_similarity_sql(image_path: &String, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error + Send>> {
    let query = r#"DELETE FROM image_similarity WHERE image_path_a = ? OR image_path_b = ?;"#;
    let r = execute_update_or_insert(pool, query, vec![ image_path, image_path ]).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL delete returned {} rows", r))))
    }
}