use std::error::Error;

use sqlx::SqlitePool;

use crate::models::image_tag::ImageTag;
use crate::database::common::execute_update_or_insert;



pub async fn execute_insert_image_tag_sql(tag: ImageTag, pool: &SqlitePool) -> Result<(), Box<dyn Error + Send>> {
    let mut column_names = ImageTag::get_meta().iter().map(|c| c.name.to_string()).collect::<Vec<String>>();
    column_names.remove(0);
    let column_names_sql = column_names.join(", ");
    let column_var_placeholders_sql = column_names.iter().map(|_| "?").collect::<Vec<&str>>().join(", ");
    let query = format!(r#"INSERT INTO image_tags ({}) VALUES ({});"#, column_names_sql, column_var_placeholders_sql);
    let params: Vec<String> = column_names.iter().filter_map(|c| tag.get_field(c)).collect();
    let params = params.iter().map(|x| x.as_str()).collect();
    let r = execute_update_or_insert(&pool, &query, params).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL insert returned {} rows", r))))
    }
}