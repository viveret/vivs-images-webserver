use std::{collections::HashSet, error::Error};

use sqlx::{Row, SqlitePool};

use crate::{database::common::execute_query, models::image_tag::{Tag, TagMetrics}};

pub async fn get_image_paths_from_tags_in_sql_db(pool: &SqlitePool) -> Result<HashSet<String>, Box<dyn Error + Send>> {
    let sql = r#"SELECT DISTINCT image_path FROM image_tags"#;
    let rows = execute_query(pool, sql, vec![]).await?;
    
    Ok(rows.iter()
        .filter_map(|r| r.try_get("image_path").ok())
        .collect())
}

pub async fn get_image_tag_names_from_db(pool: &SqlitePool) -> Result<HashSet<String>, Box<dyn Error + Send>> {
    let sql = r#"SELECT DISTINCT tag_name FROM image_tags"#;
    let rows = execute_query(pool, sql, vec![]).await?;
    
    Ok(rows.iter()
        .filter_map(|r| r.try_get("tag_name").ok())
        .collect())
}

pub async fn get_image_tags_from_db_for_path(path: &String, pool: &SqlitePool) -> Result<HashSet<Tag>, Box<dyn Error + Send>> {
    let sql = r#"SELECT tag_name, tag_label, tag_description FROM image_tags WHERE image_path = ?"#;
    let rows = execute_query(pool, sql, vec![ path ]).await?;
    
    Ok(rows.iter()
        .map(|r| Tag::new(r))
        .collect())
}

pub async fn query_image_tags_table_count(path: &String, tag: &String, pool: &SqlitePool) -> Result<usize, Box<dyn Error + Send>> {
    let sql = r#"SELECT COUNT(*) ct FROM image_tags WHERE image_path = ? AND tag_name = ?"#;
    let rows = execute_query(pool, sql, vec![ path, tag ]).await?;
    let v: Option<u32> = rows.iter().nth(0).map(|r| r.get("ct"));
    let v: usize = v.unwrap_or_default() as usize;
    Ok(v)
}

pub async fn query_image_tags_table_count_for_path(path: &str, pool: &SqlitePool) -> Result<usize, Box<dyn Error + Send>> {
    let sql = r#"SELECT COUNT(*) ct FROM image_tags WHERE image_path = ?"#;
    let rows = execute_query(pool, sql, vec![ path ]).await?;
    let v: Option<u32> = rows.iter().nth(0).map(|r| r.get("ct"));
    let v: usize = v.unwrap_or_default() as usize;
    Ok(v)
}

pub async fn query_image_tags_table_count_for_tag(tag: &str, pool: &SqlitePool) -> Result<usize, Box<dyn Error + Send>> {
    let sql = r#"SELECT COUNT(*) ct FROM image_tags WHERE tag_name = ?"#;
    let rows = execute_query(pool, sql, vec![ tag ]).await?;
    let v: Option<u32> = rows.iter().nth(0).map(|r| r.get("ct"));
    let v: usize = v.unwrap_or_default() as usize;
    Ok(v)
}

pub async fn query_all_tags(pool: &SqlitePool) -> Result<HashSet<Tag>, Box<dyn Error + Send>> {
    let sql = r#"
    SELECT tag_name, tag_label, tag_description FROM tags
    UNION SELECT tag_name, "" tag_label, "" tag_description FROM image_tags
    ORDER BY tag_name
    "#;
    let rows = execute_query(pool, sql, vec![]).await?;
    Ok(rows.iter().map(|r| Tag::new(r)).collect())
}

pub async fn query_tag(tag: &str, pool: &SqlitePool) -> Result<Option<Tag>, Box<dyn Error + Send>> {
    let sql = r#"
    SELECT tag_name, tag_label, tag_description FROM tags WHERE tag_name = ?
    "#;
    let rows = execute_query(pool, sql, vec![ tag ]).await?;
    Ok(rows.iter().nth(0).map(|r| Tag::new(r)))
}

pub async fn query_tag_metrics(tag: &str, pool: &SqlitePool) -> Result<Option<TagMetrics>, Box<dyn Error + Send>> {
    let tag_info = query_tag(tag, pool).await?;
    let tag_use_count = query_image_tags_table_count_for_tag(tag, pool).await?;
    let related_tags = query_image_tags_related_with_counts(tag, pool).await?;
    Ok(Some(TagMetrics {
        tag: tag_info.or(Some(Tag { tag_name: tag.to_string(), tag_label: String::default(), tag_description: String::default() })).unwrap(),
        use_count: tag_use_count,
        related_tags
    }))
}

pub async fn query_image_tags_related(tag: &str, pool: &SqlitePool) -> Result<HashSet<Tag>, Box<dyn Error + Send>> {
    let sql = r#"
    SELECT DISTINCT t.tag_name, 
           COALESCE(tags.tag_label, '') as tag_label, 
           COALESCE(tags.tag_description, '') as tag_description
    FROM image_tags t
    LEFT JOIN tags ON t.tag_name = tags.tag_name
    WHERE t.image_path IN (
        SELECT image_path 
        FROM image_tags 
        WHERE tag_name = ?
    )
    AND t.tag_name != ?
    ORDER BY t.tag_name
    "#;
    
    let rows = execute_query(pool, sql, vec![tag, tag]).await?;
    Ok(rows.iter().map(|r| Tag::new(r)).collect())
}

pub async fn query_image_tags_related_with_counts(tag: &str, pool: &SqlitePool) -> Result<Vec<(Tag, usize)>, Box<dyn Error + Send>> {
    let sql = r#"
    SELECT t.tag_name, 
           COALESCE(tags.tag_label, '') as tag_label, 
           COALESCE(tags.tag_description, '') as tag_description,
           COUNT(*) as co_occurrence_count
    FROM image_tags t
    LEFT JOIN tags ON t.tag_name = tags.tag_name
    WHERE t.image_path IN (
        SELECT image_path 
        FROM image_tags 
        WHERE tag_name = ?
    )
    AND t.tag_name != ?
    GROUP BY t.tag_name
    ORDER BY co_occurrence_count DESC, t.tag_name
    "#;
    
    let rows = execute_query(pool, sql, vec![tag, tag]).await?;
    Ok(rows.iter().map(|r| {
        let tag = Tag::new(r);
        let count: u32 = r.get("co_occurrence_count");
        (tag, count as usize)
    }).collect())
}