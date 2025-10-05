use actix_web::{web, App, HttpServer};

use crate::actions::worker_thread::WorkerThread;
use crate::core::data_context::WebServerActionDataContext;

pub mod api;
pub mod actions;
pub mod cache;
pub mod calc;
pub mod core;
pub mod converters;
pub mod database;
pub mod filesystem;
pub mod models;
pub mod metrics;
pub mod view;


#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let data_ctx = WebServerActionDataContext::open().await?;
    let worker_thread = WorkerThread::spawn(data_ctx.clone());
    let worker_thread_2 = worker_thread.clone();

    println!("Starting server on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data_ctx.clone()))
            .app_data(web::Data::new(data_ctx.pool.clone()))
            .app_data(web::Data::new(worker_thread_2.clone()))
            .app_data(web::Data::new(worker_thread_2.action_registry.clone()))
            .route("/", web::get().to(view::html::pages::index::index))
            .route("/actions", web::get().to(view::html::pages::actions::view_page_actions))
            .route("/actions/{action_name}", web::get().to(view::html::pages::action_detail::view_page_action_detail_get))
            .route("/actions/start/{action_name}", web::post().to(view::html::pages::action_detail::view_page_action_detail_post))
            .route("/actions/task/{action_task_id}", web::get().to(view::html::pages::task_detail::view_page_task_detail_get))
            .route("/search", web::get().to(view::html::pages::search::search_images))
            .route("/search/wallpapers", web::get().to(view::html::pages::search::search_wallpapers))
            .route("/browse/filesystem", web::get().to(view::html::pages::browse_filesystem::view_page_browse_filesystem))
            .route("/browse/by-property", web::get().to(view::html::pages::browse_by_property::view_page_browse_properties))
            .route("/browse/by-property/{property}", web::get().to(view::html::pages::browse_by_property_detail::view_page_property_details))
            .route("/browse/tags", web::get().to(view::html::pages::browse_tags::view_page_tags))
            .route("/browse/tags/{tag}", web::get().to(view::html::pages::browse_tags::view_page_tag_details))
            .route("/image", web::get().to(view::html::pages::image::view_image))
            .route("/img", web::get().to(api::web::get_image))
            .route("/style.css", web::get().to(api::web::get_style))
            .route("/api/wallpaper", web::get().to(api::api_get_wallpaper_image_path::api_get_wallpaper_image_path))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
