use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use sqlx::{Row, SqlitePool};

use crate::core::data_context::WebServerActionDataContext;
use crate::models::image::{Image, ImageFieldMeta};
use crate::database::common::execute_query;
use crate::models::image_aspect_ratio::ImageAspectRatio;
use crate::models::image_brightness::ImageBrightness;
use crate::models::image_exif::ImageExif;
use crate::models::image_iptc::ImageIptc;
use crate::models::image_ocr_text::ImageOcrText;
use crate::models::image_paths::ImagePaths;
use crate::models::image_similarity::ImageSimilarity;
use crate::models::image_xmp::ImageXmp;
use crate::models::query_params::search_params::SearchParams;

pub struct SearchBuilderImageFeature {
    pub table_name: String,
    pub columns: Vec<ImageFieldMeta>,
}

impl SearchBuilderImageFeature {
    pub fn from_meta(name: &str, columns: &[ImageFieldMeta]) -> Self {
        Self {
            table_name: name.to_string(),
            columns: columns.to_vec(),
        }
    }
}

pub struct SearchBuilder {
    select_clause_params: Vec<String>,
    criteria: Vec<(String, HashMap<String, String>)>,
    select_columns: Vec<String>,
    order_by: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
    joins: Vec<String>,
    base_table: &'static str,
    base_table_query: Option<String>,
    base_table_query_select_clause_params: Vec<String>,
    path_filter: Option<Vec<String>>,
}

impl SearchBuilder {
    pub fn new() -> Self {
        Self {
            criteria: vec![],
            select_columns: vec![],
            select_clause_params: vec![],
            order_by: None,
            limit: None,
            offset: None,
            joins: vec![],
            base_table: "image_paths",
            base_table_query: None,
            base_table_query_select_clause_params: vec![],
            path_filter: None,
        }
    }

    pub fn with_base_table(mut self, table_name: &'static str) -> Self {
        self.base_table = table_name;
        self
    }

    pub fn with_base_table_as_select(mut self, base_query: SearchBuilder) -> Self {
        // the inner base table to select, join, filter on is a nested select query
        // so base_table is an alias for the inner table and we put the inner query before the outer
        let (base_query_str, base_query_params) = base_query.build_sql();
        self.base_table_query_select_clause_params.extend_from_slice(base_query_params.as_slice());
        self.base_table_query = Some(format!("({}) ", base_query_str));
        self
    }
    
    pub fn with_criteria(mut self, criteria: Vec<(String, HashMap<String, String>)>) -> Self {
        self.criteria = criteria;
        self
    }
    
    pub fn with_order_by(mut self, order_by: &str) -> Self {
        self.order_by = Some(order_by.to_string());
        self
    }
    
    pub fn with_pagination(mut self, limit: i32, offset: i32) -> Self {
        self.limit = Some(limit);
        self.offset = Some(offset);
        self
    }
    
    pub fn with_limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }
    
    pub fn with_offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }
    
    pub fn with_join(mut self, join_clause: &str) -> Self {
        self.joins.push(join_clause.to_string());
        self
    }

    pub fn with_select_clause_param(mut self, param_string: String) -> Self {
        self.select_clause_params.push(param_string);
        self
    }
    
    pub fn with_tables(mut self, join_tables: Vec<SearchBuilderImageFeature>) -> Self {
        let base_table = self.base_table;
        for table in join_tables {
            self = self.with_field_meta_columns(table.columns);
            self = self.with_join(&format!("LEFT JOIN {} ON {}.image_path = {}.image_path", 
                table.table_name, base_table, table.table_name));
        }
        self
    }

    pub fn with_field_meta_columns(mut self, columns: Vec<ImageFieldMeta>) -> Self {
        let select_columns: Vec<String> = columns
            .iter().map(|c| format!("[{}].[{}]", c.table_name, c.name)).collect();
        self.select_columns.extend_from_slice(&select_columns);
        self
    }

    pub fn with_default_tables(self) -> Self {
        let base_table_meta = match self.base_table {
            "image_paths" => ImagePaths::get_meta(),
            "image_similarity" => ImageSimilarity::get_meta_for_single(),
            _ => panic!("uknown base table {}", self.base_table),
        };
        println!("base_table: {}", self.base_table);
        let default_tables = vec![
            SearchBuilderImageFeature::from_meta("image_exif", &ImageExif::get_meta()[1..]),
            SearchBuilderImageFeature::from_meta("image_brightness", &ImageBrightness::get_meta()[1..]),
            SearchBuilderImageFeature::from_meta("image_ocr_text", &ImageOcrText::get_meta()[1..]),
            SearchBuilderImageFeature::from_meta("image_aspect_ratio", &ImageAspectRatio::get_meta()[1..]),
            SearchBuilderImageFeature::from_meta("image_iptc", &ImageIptc::get_meta()[1..]),
            SearchBuilderImageFeature::from_meta("image_xmp", &ImageXmp::get_meta()[1..]),
        ];
        self.with_field_meta_columns(base_table_meta).with_tables(default_tables)
    }

    pub fn with_select_columns(mut self, columns: Vec<String>) -> Self {
        self.select_columns.extend_from_slice(columns.as_slice());
        self
    }

    pub fn with_get_paths(mut self, paths: Vec<String>) -> Self {
        self.path_filter = Some(paths);
        self
    }

    pub fn get_tables_selected(&self) -> Vec<String> {
        self.select_columns.iter().filter_map(|c: &String| {
            let parts: Vec<&str> = c.split('.').collect();
            if parts.len() == 2 {
                // Remove brackets from table name
                let table_name = parts[0].trim_matches(['[', ']']).trim();
                if !table_name.is_empty() {
                    return Some(table_name.to_string());
                }
            }
            None
        }).collect()
    }

    pub fn build_sql(&self) -> (String, Vec<String>) {
        let mut params = self.select_clause_params.clone();
        if !self.base_table_query_select_clause_params.is_empty() {
            params.extend_from_slice(self.base_table_query_select_clause_params.as_slice());
        }
        
        // Build SELECT clause
        let select_columns_sql = if self.select_columns.is_empty() {
            "*".to_string()
        } else {
            self.select_columns.join(", ")
        };
        
        let mut select_clause = format!(
            "SELECT {} FROM {}{}",
            select_columns_sql, self.base_table_query.as_ref().map(|s| s.as_str()).unwrap_or_default(), self.base_table
        );
        
        // Add joins
        for join in &self.joins {
            select_clause.push_str("\n  ");
            select_clause.push_str(join);
        }

        let where_clause = self.build_where_clause(&mut params);
    
        let order_clause = self.order_by
            .as_ref()
            .map(|ob| format!("ORDER BY {}", ob))
            .unwrap_or_default();
        
        let limit_clause = self.limit
            .map(|l| format!("LIMIT {}", l))
            .unwrap_or_default();
        
        let offset_clause = self.offset
            .map(|o| format!("OFFSET {}", o))
            .unwrap_or_default();
        
        let sql = format!(
            "{} {} {} {} {}",
            select_clause, where_clause, order_clause, limit_clause, offset_clause
        );
        
        (sql, params)
    }
    
    fn build_where_clause(&self, params: &mut Vec<String>) -> String {
        let mut where_parts = Vec::new();
        let mut criteria_params = Vec::new();

        // Build criteria-based WHERE conditions
        let mut query_criteria_sql = String::new();
        for (field_op, field_group) in &self.criteria {
            if !field_group.is_empty() {
                query_criteria_sql.push_str(" AND (");
                let mut inner_sql = String::new();
                for (field, value) in field_group {
                    if !value.is_empty() {
                        inner_sql.push_str(&format!(" {} {}", field_op, field));
                        criteria_params.push(value.clone());
                    }
                }
                // Remove first " {field_op} "
                if inner_sql.len() > field_op.len() + 2 {
                    inner_sql = inner_sql[field_op.len() + 2..].to_string();
                }
                query_criteria_sql.push_str(&inner_sql);
                query_criteria_sql.push_str(")");
            }
        }

        // Add path filter if specified
        if let Some(paths) = &self.path_filter {
            if !paths.is_empty() {
                let placeholders = vec!["?"; paths.len()].join(", ");
                where_parts.push(format!("{}.image_path IN ({})", self.base_table, placeholders));
                params.extend(paths.clone());
            }
        }

        // Add criteria-based conditions
        if !query_criteria_sql.is_empty() {
            // Remove leading " AND " and add to where_parts
            let criteria_sql = &query_criteria_sql[5..];
            where_parts.push(criteria_sql.to_string());
            params.extend(criteria_params);
        }

        if !where_parts.is_empty() {
            format!(" WHERE {}", where_parts.join(" AND "))
        } else {
            String::new()
        }
    }

    pub async fn execute<T, F>(self, pool: WebServerActionDataContext, row_handler: F) -> Result<Vec<T>, Box<dyn Error + Send>>
    where
        F: Fn(WebServerActionDataContext, &sqlx::sqlite::SqliteRow) -> T,
    {
        let (sql, params) = self.build_sql();
        let param_refs: Vec<&str> = params.iter().map(|s| s.as_str()).collect();
        
        let results = execute_query(&pool.pool, &sql, param_refs).await?;
        Ok(results.into_iter().map(|row| row_handler(pool.clone(), &row)).collect())
    }

    pub async fn execute_async<T, F, Fut>(self, pool: WebServerActionDataContext, row_handler: F) -> Result<Vec<T>, Box<dyn Error + Send>>
    where
        F: Fn(WebServerActionDataContext, sqlx::sqlite::SqliteRow) -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let (sql, params) = self.build_sql();
        let param_refs: Vec<&str> = params.iter().map(|s| s.as_str()).collect();
        
        let results = execute_query(&pool.pool, &sql, param_refs).await?;
        
        let mut handles = Vec::new();
        for row in results {
            handles.push(row_handler(pool.clone(), row));
        }
        
        let processed_results = futures::future::join_all(handles).await;
        Ok(processed_results)
    }

    pub async fn execute_get_images_and<T, F, Fut>(self, pool: WebServerActionDataContext, image_handler: F) -> Result<Vec<T>, Box<dyn Error + Send>>
    where
        F: Fn(WebServerActionDataContext, sqlx::sqlite::SqliteRow, Result<Image, Box<dyn Error + Send>>) -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let tables_selected = self.get_tables_selected();
        let results = futures::future::join_all(self.execute_async(pool.clone(), async |pool, row| {
            let x = transform_sql_row_to_image(pool.clone(), &row, tables_selected.clone()).await;
            image_handler(pool,
                row,
                x
            )
        }).await?).await;
        Ok(results)
    }

    pub async fn execute_get_images(self, pool: WebServerActionDataContext) -> Result<Vec<Result<Image, Box<dyn Error + Send>>>, Box<dyn Error + Send>> {
        self.execute_get_images_and(pool, async |_, _, image| image).await
    }

    pub async fn execute_count(self, pool: &SqlitePool) -> Result<i64, Box<dyn Error + Send>> {
        let (sql, params) = self.build_sql();
        let count_sql = format!("SELECT COUNT(*) as count FROM ({}) as subquery", sql);
        let param_refs: Vec<&str> = params.iter().map(|s| s.as_str()).collect();
        
        let results = execute_query(pool, &count_sql, param_refs).await?;
        if let Some(row) = results.first() {
            let count: i64 = row.try_get("count").unwrap_or(0);
            Ok(count)
        } else {
            Ok(0)
        }
    }
}

// get count without fetching full image data
pub async fn count_sql_db_images_by_criteria(
    pool: &SqlitePool,
    criteria: &Vec<(String, HashMap<String, String>)>,
) -> Result<i64, Box<dyn Error + Send>> {
    let builder = SearchBuilder::new()
        .with_default_tables()
        .with_criteria(criteria.clone());

    builder.execute_count(pool).await
}

pub async fn execute_search_images_query_with_criteria(
    pool: WebServerActionDataContext,
    criteria: &Vec<(String, HashMap<String, String>)>,
    order_by: Option<&str>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<Result<Image, Box<dyn Error + Send>>>, Box<dyn Error + Send>> {
    let mut builder = SearchBuilder::new()
        .with_default_tables()
        .with_criteria(criteria.clone());

    if let Some(order) = order_by {
        builder = builder.with_order_by(order);
    }

    if let Some(lim) = limit {
        builder = builder.with_limit(lim);
    }

    if let Some(offset) = offset {
        builder = builder.with_offset(offset);
    }

    let tables_selected = builder.get_tables_selected();
    builder.execute_async(pool, async |pool, row| transform_sql_row_to_image(pool, &row, tables_selected.clone()).await).await
}

// this needs to be fixed to allow adding additional "features" / tables / other things
// for instance, similarity
async fn transform_sql_row_to_image(
    pool: WebServerActionDataContext, 
    row: &sqlx::sqlite::SqliteRow,
    tables_selected: Vec<String>
) -> Result<Image, Box<dyn Error + Send>> {
    let mut img = Image::new(&row, tables_selected);
    
    if let Some(thumb) = pool.get_thumbnail_at_most_width_length(&img.path, 64).await? {
        img.assign_thumbnail(thumb);
    }
    
    Ok(img)
}

async fn transform_results_to_output(
    pool: WebServerActionDataContext, 
    results: Vec<sqlx::sqlite::SqliteRow>,
    tables_selected: Vec<String>
) -> Result<Vec<Image>, Box<dyn Error + Send>> {
    let futures: Vec<_> = results
        .into_iter()
        .map(async |row| transform_sql_row_to_image(pool.clone(), &row, tables_selected.clone()).await)
        .collect();
    
    futures::future::try_join_all(futures).await
}

pub async fn search_images_by_criteria(
    pool: WebServerActionDataContext,
    params: &SearchParams,
    order_by: Option<&str>,
) -> Result<SearchImagesPageModel, Box<dyn Error + Send>> {
    let criteria = params.into_sql_query_params();
    
    let total_count = count_sql_db_images_by_criteria(&pool.pool, &criteria).await? as usize;
    
    let items = execute_search_images_query_with_criteria(pool, &criteria, order_by, params.get_limit(), params.get_offset())
        .await?;
    
    Ok(SearchImagesPageModel { total_count, items })
}

pub struct SearchImagesPageModel {
    pub total_count: usize,
    pub items: Vec<Result<Image, Box<dyn Error + Send>>>,
}

pub async fn find_image_by_path(pool: WebServerActionDataContext, path: &str) -> Result<Option<Image>, Box<dyn Error + Send>> {
    let mut params = HashMap::new();
    params.insert("image_paths.image_path = ?".to_string(), path.to_string());
    let criteria = vec![ ("".to_string(), params) ];
    let results = execute_search_images_query_with_criteria(pool, &criteria, None, Some(1), None).await?;
    match results.first() {
        Some(Ok(item)) => {
            Ok(Some(item.clone()))
        }
        Some(Err(e)) => {
            Err(Box::new(std::io::Error::other(e.to_string())))
        }
        _ => Ok(None)
    }
}

pub async fn get_images_by_paths(
    pool: WebServerActionDataContext, 
    paths: Vec<String>
) -> Result<Vec<Image>, Box<dyn Error + Send>> {
    let builder = SearchBuilder::new()
        .with_default_tables()
        .with_get_paths(paths);

    let (query, params) = builder.build_sql();
    let params_str = params.iter().map(|s| s.as_str()).collect();
    let results = execute_query(&pool.pool, &query, params_str).await?;
    let tables_selected = builder.get_tables_selected();
    transform_results_to_output(pool, results, tables_selected).await
}

// Additional helper function that combines path filtering with criteria
pub async fn get_images_by_paths_with_criteria(
    pool: WebServerActionDataContext,
    paths: Vec<String>,
    criteria: &Vec<(String, HashMap<String, String>)>,
    order_by: Option<&str>,
) -> Result<Vec<Image>, Box<dyn Error + Send>> {
    let mut builder = SearchBuilder::new()
        .with_default_tables()
        .with_get_paths(paths)
        .with_criteria(criteria.clone());

    if let Some(order) = order_by {
        builder = builder.with_order_by(order);
    }

    let (query, params) = builder.build_sql();
    let params_str = params.iter().map(|s| s.as_str()).collect();
    let results = execute_query(&pool.pool, &query, params_str).await?;
    let tables_selected = builder.get_tables_selected();
    transform_results_to_output(pool, results, tables_selected).await
}