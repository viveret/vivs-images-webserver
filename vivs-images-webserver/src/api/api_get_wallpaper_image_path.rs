use std::error::Error;

use actix_web::HttpResponse;

use crate::core::data_context::WebServerActionDataContext;
use crate::models::query_params::default_search_params::get_image_wallpaper_based_on_brightness_search_params;
use crate::database::query::search::search_images_by_criteria;



pub async fn api_get_wallpaper_image_path_inner(pool: WebServerActionDataContext) -> Result<Option<String>, Box<dyn Error + Send>> {
    let wallpaper_search_params = get_image_wallpaper_based_on_brightness_search_params();
    let wallpaper_search = search_images_by_criteria(pool, &wallpaper_search_params, Some("RANDOM()")).await?;
    if wallpaper_search.total_count > 0 {
        if let Some(wallpaper) = wallpaper_search.items.first() {
            match wallpaper {
                Ok(wallpaper) => {
                    return Ok(Some(wallpaper.path.clone()));
                }
                Err(e) => {
                    return Err(Box::new(std::io::Error::other(e.to_string())) as Box<dyn Error + Send>);
                }
            }
        }
    }
    
    Ok(None)
}

pub async fn api_get_wallpaper_image_path(pool: actix_web::web::Data<WebServerActionDataContext>) -> Result<HttpResponse, actix_web::Error> {
    api_get_wallpaper_image_path_inner(pool.get_ref().clone()).await
        .map(|wallpaper| {
            if let Some(w) = wallpaper {
                let json = serde_json::json!({ "wallpaper": w });
                return Ok(HttpResponse::Ok().content_type("application/json").body(json.to_string()));
            } else {
                Ok(HttpResponse::NotFound().body(format!("Wallpaper image not found")))
            }
        })
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
}