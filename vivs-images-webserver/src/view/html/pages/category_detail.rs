use std::collections::HashMap;

use actix_web::{web, HttpResponse, Result};
use sqlx::{Row, SqlitePool};

use crate::{database::{common::{execute_query, CATEGORIES}, query::search::{execute_search_images_query_with_criteria, search_images_by_criteria}}, models::{image::Image, query_params::search_params::SearchParams}, view::html::{common::create_html_table, layout::layout_view, model_views::image::generate_image_table_rows, search::generate_category_filter_form}};


pub async fn category_detail(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>,
    params: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse> {
    let category = path.into_inner();

    if !CATEGORIES.contains(&category.as_str()) {
        return Ok(HttpResponse::NotFound().body("Category not found"));
    }

    let mut content = String::new();
    let filter_value = params.get("value").cloned().unwrap_or_default();

    // Show filter form for this category
    content.push_str(&generate_category_filter_form(&category));

    if !filter_value.is_empty() {
        let mut criteria = HashMap::new();
        criteria.insert(category.to_string(), filter_value.to_string());
        let criteria_list  = vec![("".to_string(), criteria)];
        let rows = execute_search_images_query_with_criteria(pool.get_ref(), &criteria_list, Some("image_taken_at DESC"), Some(100), Some(0))
            .await?;

        if !rows.is_empty() {
            let columns = ["thumbnail", "path", "image_taken_at", "camera_model", "lens_model", "exposure_time", "iso", "focal_length"];
            let columns = columns.map(String::from).to_vec();
            let rows_html = generate_image_table_rows(&rows, &columns);
            let table_html = create_html_table(
                &format!("Images with {}: {}", category, filter_value),
                &SearchParams::get_column_titles(&columns),
                &rows_html
            );
            content.push_str(&table_html);
        } else {
            content.push_str(&format!("<p>No images found for {}: {}</p>", category, filter_value));
        }
    } else {
        // Show distinct values for this category
        let distinct_query = format!(
            "SELECT DISTINCT {} as value, COUNT(*) as count FROM image_exif WHERE {} IS NOT NULL GROUP BY {} ORDER BY count DESC LIMIT 50",
            category, category, category
        );

        let rows = execute_query(pool.get_ref(), &distinct_query, vec![]).await?;

        if !rows.is_empty() {
            let mut values_html = String::from("<div class='value-list'>");
            for row in rows {
                let value: String = row.try_get("value").unwrap_or_default();
                let count: i64 = row.try_get("count").unwrap_or(0);

                values_html.push_str(&format!(
                    r#"<a href="/categories/{}?value={}" class="value-item">{} ({})</a>"#,
                    category, value, value, count
                ));
            }
            values_html.push_str("</div>");
            content.push_str(&values_html);
        }
    }

    let html = layout_view(Some(&category), &content);
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
