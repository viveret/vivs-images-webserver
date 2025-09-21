use actix_web::{web, App, HttpResponse, HttpServer, Result};
use sqlx::SqlitePool;
use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::HttpRequest;
use std::io;
use std::collections::HashMap;

use crate::actions::worker_thread::WorkerThread;


pub mod actions;
pub mod converters;
pub mod database;
pub mod filesystem;
pub mod models;
pub mod metrics;
pub mod view;


async fn get_image(req: HttpRequest, path: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    if let Some(image_path) = path.get("path") {
        let path_buf = PathBuf::from(image_path);

        // Security check: prevent directory traversal attacks
        if path_buf.components().any(|comp| comp == std::path::Component::ParentDir) {
            return Ok(HttpResponse::BadRequest().body("Invalid path"));
        }

        match NamedFile::open(&path_buf) {
            Ok(file) => {
                // Determine content type based on file extension
                let content_type = match path_buf.extension().and_then(|ext| ext.to_str()) {
                    Some("jpg") | Some("jpeg") => "image/jpeg",
                    Some("png") => "image/png",
                    Some("gif") => "image/gif",
                    Some("webp") => "image/webp",
                    Some("tiff") | Some("tif") => "image/tiff",
                    _ => "application/octet-stream",
                };

                Ok(file
                    .use_last_modified(true)
                    .use_etag(true)
                    .set_content_type(content_type.parse().unwrap())
                    .into_response(&req))
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                Ok(HttpResponse::NotFound().body(format!("Image {} not found", image_path)))
            }
            Err(e) => {
                eprintln!("Error serving image {}: {}", image_path, e);
                Ok(HttpResponse::InternalServerError().body("Error serving image"))
            }
        }
    } else {
        Ok(HttpResponse::BadRequest().body("Missing path parameter"))
    }
}

pub fn get_file_from_exe_dir(path: &str) -> PathBuf {
    std::env::current_dir().unwrap().join("vivs-images-webserver/").join(path)
}

async fn get_style(req: HttpRequest) -> Result<HttpResponse> {
    let stylesheet_path = get_file_from_exe_dir("style.css");
    match NamedFile::open(stylesheet_path.clone()) {
        Ok(file) => {
            Ok(file
                .use_last_modified(true)
                .use_etag(true)
                .set_content_type("text/css".parse().unwrap())
                .into_response(&req))
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            Ok(HttpResponse::NotFound().body(format!("Stylesheet {:?} not found", stylesheet_path)))
        }
        Err(e) => {
            eprintln!("Error serving image {:?}: {}", stylesheet_path, e);
            Ok(HttpResponse::InternalServerError().body("Error serving image"))
        }
    }
}

pub const DB_FILE: &str = "/home/viveret/vivs-images.db";

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Connect to SQLite database
    let pool = SqlitePool::connect(&format!("sqlite:{}", DB_FILE))
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;

    let worker_thread = WorkerThread::spawn(pool.clone());
    let worker_thread_2 = worker_thread.clone();

    println!("Starting server on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(worker_thread_2.clone()))
            .app_data(web::Data::new(worker_thread_2.action_registry.clone()))
            .route("/", web::get().to(view::html::pages::index::index))
            .route("/actions", web::get().to(view::html::pages::actions::view_page_actions))
            .route("/actions/{action_name}", web::get().to(view::html::pages::action_detail::view_page_action_detail_get))
            .route("/actions/start/{action_name}", web::post().to(view::html::pages::action_detail::view_page_action_detail_post))
            .route("/actions/task/{action_task_id}", web::get().to(view::html::pages::task_detail::view_page_task_detail_get))
            .route("/search", web::get().to(view::html::pages::search::search_images))
            .route("/browse", web::get().to(view::html::pages::browse_all::browse_all))
            .route("/categories", web::get().to(view::html::pages::categories::view_page_categories))
            .route("/categories/{category}", web::get().to(view::html::pages::category_detail::category_detail))
            .route("/image", web::get().to(view::html::pages::image::view_image))
            .route("/img", web::get().to(get_image))
            .route("/style.css", web::get().to(get_style))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
