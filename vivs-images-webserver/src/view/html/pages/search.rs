use actix_web::http::header::LOCATION;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use sqlx::SqlitePool;

use crate::core::data_context::WebServerActionDataContext;
use crate::models::query_params::default_search_params::get_image_wallpaper_based_on_brightness_search_params;
use crate::view::html::model_views::image::generate_image_table_rows;
use crate::view::html::layout::layout_view;
use crate::view::html::common::create_html_table;
use crate::models::query_params::search_params::SearchParams;
use crate::database::query::search::search_images_by_criteria;
use crate::view::html::model_views::search_params_advanced::search_images_advanced_form;
use crate::view::html::model_views::search_params_simple::search_images_simple_form;


pub async fn search_images(
    pool: web::Data<WebServerActionDataContext>,
    req: HttpRequest
) -> Result<HttpResponse> {
    let params = SearchParams::new_from_querystring(req.query_string());
    let search_form = if params.get_use_simple_view().unwrap_or_default() {
        search_images_simple_form(&pool.get_ref().pool, &params).await
    } else {
        search_images_advanced_form(&pool.get_ref().pool, &params).await
    }?;

    let image_search = search_images_by_criteria(pool.get_ref().clone(), &params, Some("image_taken_at DESC"))
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let params_title = params.to_string();
    let columns_default = ["thumbnail", "name", "ocr_text", "camera_model", "image_description", "brightness", "image_taken_at"];
    let columns_default = columns_default.map(String::from).to_vec();
    let columns = params.get_columns_to_display().unwrap_or(columns_default);
    let column_titles = SearchParams::get_column_titles(&columns);

    let title = format!("{} results for '{}'", image_search.total_count, params_title);
    let rows_html = generate_image_table_rows(&image_search.items, &columns);
    let table_html = create_html_table(
        &title,
        &column_titles,
        &rows_html
    );

    let mut content_html = String::new();
    content_html.push_str(&search_form);
    content_html.push_str(&table_html);

    let html = layout_view(Some(&title), &content_html);
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}



pub async fn search_wallpapers() -> Result<HttpResponse> {
    let params = get_image_wallpaper_based_on_brightness_search_params();
    let params = params.into_html_params();
    let query_string = serde_urlencoded::to_string(&params).unwrap();
    let href = format!("/search?{}", query_string);
    Ok(HttpResponse::SeeOther().insert_header((LOCATION, href)).finish())
}