use std::error::Error;

use sqlx::{Pool, Sqlite};

use crate::models::image_iptc::ImageIptc;
use crate::database::common::{execute_update_or_insert, execute_update_or_insert_with_nulls};


pub async fn execute_update_image_iptc_sql(iptc: &ImageIptc, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error + Send>> {
    let mut column_names = ImageIptc::get_meta().iter().map(|c| c.name.to_string()).collect::<Vec<String>>();
    _ = column_names.remove(0);

    let column_names_sql = column_names.iter().map(|c| format!("{} = ?", c)).collect::<Vec<String>>().join(", ");

    let query = format!(r#"
        UPDATE image_iptc
        SET {}, updated_at = CURRENT_TIMESTAMP
        WHERE image_path = ?;
    "#, column_names_sql);

    let mut params: Vec<Option<String>> = column_names.iter().map(|c| iptc.get_field(c)).collect();
    params.push(Some(iptc.image_path.to_string()));

    let r = execute_update_or_insert_with_nulls(pool, &query, params).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL update iptc returned {} rows", r))))
    }
}

pub async fn execute_insert_image_iptc_sql(iptc: ImageIptc, pool: Pool<Sqlite>) -> Result<(), Box<dyn Error + Send>> {
    let column_names = ImageIptc::get_meta().iter().map(|c| c.name.to_string()).collect::<Vec<String>>();
    let column_names_sql = column_names.join(", ");
    let column_var_placeholders_sql = column_names.iter().map(|_| "?").collect::<Vec<&str>>().join(", ");
    let query = format!(r#"INSERT INTO image_iptc ({}) VALUES ({});"#, column_names_sql, column_var_placeholders_sql);
    let params      = column_names.iter().map(|c| iptc.get_field(c)).collect();
    let r = execute_update_or_insert_with_nulls(&pool, &query, params).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL insert iptc returned {} rows", r))))
    }
}

pub async fn execute_delete_image_iptc_sql(image_path: &String, pool: &Pool<Sqlite>) -> Result<(), Box<dyn Error + Send>> {
    let query = r#"DELETE FROM image_iptc WHERE image_path = ?;"#;
    let params      = vec![ image_path.as_str() ];
    let r = execute_update_or_insert(pool, query, params).await?;
    let r = r.rows_affected();
    if r == 1 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::other(format!("SQL delete iptc returned {} rows", r))))
    }
}