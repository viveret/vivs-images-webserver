use std::error::Error;

use sqlx::SqlitePool;

use crate::database::common::execute_update_or_insert;



pub async fn execute_insert_image_path_sql(image_path: &String, pool: &SqlitePool) -> Result<(), Box<dyn Error + Send>> {
    let query = r#"
        INSERT INTO image_paths(image_path) VALUES(?);
    "#;
    let r = execute_update_or_insert(pool, query, vec![ image_path ]).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL update returned {} rows", r))))
    }
}