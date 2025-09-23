use sqlx::{Row, SqlitePool};

use crate::database::common::execute_query;



// Retrieves exif image paths from the exif table in the database
pub async fn get_image_paths_from_db(pool: &SqlitePool) -> actix_web::Result<Vec<String>> {
    let sql = r#"SELECT image_path FROM image_exif"#;
    let rows = execute_query(pool, sql, vec![]).await?;
    
    Ok(rows.iter()
        .filter_map(|r| r.try_get("image_path").ok())
        .collect())
}