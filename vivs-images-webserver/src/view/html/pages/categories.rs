use std::collections::HashMap;

use actix_web::{web, HttpResponse, Result};
use sqlx::SqlitePool;

use crate::database::query::search::{execute_search_images_query_with_criteria, search_images_by_criteria};
use crate::models::query_params::search_params::SearchParams;
use crate::database::common::CATEGORIES;
use crate::view::html::common::create_html_table;
use crate::view::html::layout::layout_view;
use crate::view::html::model_views::image::generate_image_table_rows;
use crate::view::html::search::generate_category_filter_form;


// Generate category browsing interface
fn generate_category_interface() -> String {
    let mut html = String::from(r#"
    <div class="categories">
        <h2>Browse by Category</h2>
        <div class="category-list">
    "#);

    for category in CATEGORIES {
        let display_name = match *category {
            "camera_model" => "Camera Model",
            "camera_make" => "Camera Make",
            "lens_model" => "Lens Model",
            "iso_speed" => "ISO Speed",
            "focal_length" => "Focal Length",
            _ => category,
        };

        html.push_str(&format!(
            r#"<div class="category-item">
                <a href="/categories/{}">{}</a>
               </div>"#,
            category, display_name
        ));
    }

    html.push_str("</div></div>");
    html
}


pub async fn view_page_categories(
    pool: web::Data<SqlitePool>,
    params: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse> {
    let category_interface = generate_category_interface();
    let mut content = category_interface;

    // If a specific category filter is provided, show results
    let params = SearchParams::new_from_hashmap(&params);
    if let (Some(category), Some(value)) = (&params.get_category(), &params.get_category_value()) {
        if CATEGORIES.contains(&category.as_str()) {
            let mut criteria = HashMap::new();
            criteria.insert(category.to_string(), value.to_string());
            let criteria_list = vec![("".to_string(), criteria)];
            let rows = execute_search_images_query_with_criteria(pool.get_ref(), &criteria_list, Some("image_taken_at DESC"), params.get_limit(), params.get_offset())
                .await?;

            if !rows.is_empty() {
                let columns = ["thumbnail", "path", "image_taken_at", "camera_model", "lens_model", "exposure_time", "iso", "focal_length"];
                let columns = columns.map(String::from).to_vec();
                let rows_html = generate_image_table_rows(&rows, &columns);
                let filter_form = generate_category_filter_form(category);
                let table_html = create_html_table(
                    &format!("Images with {}: {}", category, value),
                    &SearchParams::get_column_titles(&columns),
                    &rows_html
                );

                content = format!("{}{}{}", filter_form, table_html, content);
            } else {
                content = format!("{}<p>No images found for {}: {}</p>", content, category, value);
            }
        }
    }

    let html = layout_view(Some("Browse by Category"), &content);
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}