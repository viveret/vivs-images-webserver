use actix_web::{web, HttpResponse, Result};
use sqlx::SqlitePool;

use crate::{database::common::execute_query, models::query_params::search_params::SearchParams, view::html::{common::create_html_table, layout::layout_view, model_views::image::generate_image_table_rows}};


pub async fn browse_all(pool: web::Data<SqlitePool>) -> Result<HttpResponse> {
    let rows = execute_query(pool.get_ref(),
        "SELECT * FROM image_exif ORDER BY image_taken_at DESC LIMIT 100",
        vec![]
    ).await?;

    let rows: Vec<_> = rows.into_iter().map(|row| {
        crate::models::image::Image::new(&row)
    }).collect();

    let columns = ["thumbnail", "path", "image_taken_at", "camera_model", "lens_model", "exposure_time", "iso", "focal_length"];
    let columns = columns.map(String::from).to_vec();
    let rows_html = generate_image_table_rows(&rows, &columns);
    let table_html = create_html_table(
        "Browse All Images",
        &SearchParams::get_column_titles(&columns),
        &rows_html
    );

    let html = layout_view(Some("Browse All"), &table_html);
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

