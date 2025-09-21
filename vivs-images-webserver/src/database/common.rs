use sqlx::{sqlite::SqliteQueryResult, SqlitePool};


// Available categories for browsing
pub const CATEGORIES: &[&str] = &[
    "camera_model", "camera_make", "lens_model", "iso_speed", "focal_length"
];

// Helper function to execute SQL queries and handle errors
pub async fn execute_query(pool: &SqlitePool, query: &str, params: Vec<&str>) -> Result<Vec<sqlx::sqlite::SqliteRow>, actix_web::Error> {
    println!("Executing query: {}", query);
    println!("With params: {:?}", params);

    let mut query_builder = sqlx::query(query);

    for param in params {
        query_builder = query_builder.bind(param);
    }

    // println!("Final query: {}", query_builder.sql());

    query_builder
        .fetch_all(pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)
}


// Helper function to execute SQL insert / update and handle errors
pub async fn execute_update_or_insert(pool: &SqlitePool, query: &str, params: Vec<&str>) -> Result<SqliteQueryResult, actix_web::Error> {
    println!("Executing update or insert: {}", query);
    println!("With params: {:?}", params);

    let mut query_builder = sqlx::query(query);

    for param in params {
        query_builder = query_builder.bind(param);
    }

    query_builder
        .execute(pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)
}


pub async fn execute_update_or_insert_with_nulls(pool: &SqlitePool, query: &str, params: Vec<Option<String>>) -> Result<SqliteQueryResult, actix_web::Error> {
    println!("Executing update or insert with nulls: {}", query);
    println!("With params: {:?}", params);

    let mut query_builder = sqlx::query(query);

    for param in params {
        query_builder = query_builder.bind(param);
    }

    query_builder
        .execute(pool)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)
}