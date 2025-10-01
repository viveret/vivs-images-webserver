use actix_web::HttpResponse;
use sqlx::SqlitePool;

use crate::models::query_params::default_search_params::get_image_wallpaper_based_on_brightness_search_params;
use crate::database::query::search::search_images_by_criteria;



pub async fn api_get_wallpaper_image_path_inner(pool: SqlitePool) -> Result<Option<String>, actix_web::Error> {
    let wallpaper_search_params = get_image_wallpaper_based_on_brightness_search_params();
    let wallpaper_search = search_images_by_criteria(&pool, &wallpaper_search_params, Some("RANDOM()")).await?;
    if wallpaper_search.total_count > 0 {
        if let Some(wallpaper) = wallpaper_search.items.first() {
            return Ok(Some(wallpaper.path.clone()));
        }
    }
    
    Ok(None)
}

pub async fn api_get_wallpaper_image_path(pool: actix_web::web::Data<SqlitePool>) -> Result<HttpResponse, actix_web::Error> {
    api_get_wallpaper_image_path_inner((**pool).clone()).await
        .map(|wallpaper| {
            if let Some(w) = wallpaper {
                let json = serde_json::json!({ "wallpaper": w });
                return Ok(HttpResponse::Ok().content_type("application/json").body(json.to_string()));
            } else {
                Ok(HttpResponse::NotFound().body(format!("Wallpaper image not found")))
            }
        })?
}