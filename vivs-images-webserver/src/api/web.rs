use actix_web::{web, HttpResponse, Result};
use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::HttpRequest;
use std::io;
use std::collections::HashMap;


pub async fn get_image(req: HttpRequest, path: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
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

pub async fn get_style(req: HttpRequest) -> Result<HttpResponse> {
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