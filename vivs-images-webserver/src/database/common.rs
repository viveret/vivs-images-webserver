use std::error::Error;

use actix_web::Either;
use sqlx::{sqlite::SqliteQueryResult, SqlitePool};


pub const PRINTLN_DEBUG: bool = true;

// Helper function to execute SQL queries and handle errors
pub async fn execute_query(pool: &SqlitePool, query: &str, params: Vec<&str>) -> Result<Vec<sqlx::sqlite::SqliteRow>, Box<dyn Error + Send>> {
    if PRINTLN_DEBUG {
        println!("Executing query: {}", query);
        println!("With params: {:?}", params);
    }

    let mut query_builder = sqlx::query(query);

    for param in params {
        query_builder = query_builder.bind(param);
    }

    // println!("Final query: {}", query_builder.sql());

    query_builder
        .fetch_all(pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send>)
}


// Helper function to execute SQL insert / update and handle errors
pub async fn execute_update_or_insert(pool: &SqlitePool, query: &str, params: Vec<&str>) -> Result<SqliteQueryResult, Box<dyn Error + Send>> {
    if PRINTLN_DEBUG {
        println!("Executing update or insert: {}", query);
        println!("With params: {:?}", params);
    }

    let mut query_builder = sqlx::query(query);

    for param in params {
        query_builder = query_builder.bind(param);
    }

    query_builder
        .execute(pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send>)
}

pub async fn execute_update_or_insert_with_blob(pool: &SqlitePool, query: &str, params: Vec<Either<&str, Vec<u8>>>) -> Result<SqliteQueryResult, Box<dyn Error + Send>> {
    if PRINTLN_DEBUG {
        println!("Executing update or insert blob: {}", query);
    }

    let mut query_builder = sqlx::query(query);

    for param in params {
        query_builder = match param {
            Either::Left(s) => query_builder.bind(s),
            Either::Right(blob) => query_builder.bind(blob),
        };
    }

    query_builder
        .execute(pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send>)
}


pub async fn execute_update_or_insert_with_nulls(pool: &SqlitePool, query: &str, params: Vec<Option<String>>) -> Result<SqliteQueryResult, Box<dyn Error + Send>> {
    if PRINTLN_DEBUG {
        println!("Executing update or insert with nulls: {}", query);
        println!("With params: {:?}", params);
    }

    let mut query_builder = sqlx::query(query);

    for param in params {
        query_builder = query_builder.bind(param);
    }

    query_builder
        .execute(pool)
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send>)
}