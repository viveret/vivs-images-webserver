use actix_web::{web, HttpResponse, Result};
use sqlx::Row;

use crate::models::image::Image;
use crate::view::html::layout::layout_view;
use crate::database::common::execute_query;
use crate::core::data_context::WebServerActionDataContext;


pub async fn view_page_property_details(
    pool: web::Data<WebServerActionDataContext>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let property = path.into_inner();

    let mut content = String::new();

    // Show distinct values for this property
    let meta = Image::get_meta_field(&property);
    let table_name = meta.map(|x| x.table_name).unwrap_or_default();
    let distinct_query = format!(
        "SELECT DISTINCT {} as value, COUNT(*) as count FROM {} WHERE {} IS NOT NULL GROUP BY {} ORDER BY count DESC LIMIT 50",
        property, table_name, property, property
    );

    let rows = execute_query(&pool.get_ref().pool, &distinct_query, vec![]).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    if !rows.is_empty() {
        let mut values_html = String::from("<div class='value-list'>");
        for row in rows {
            let value_u32: Option<u32> = row.try_get("value").ok();
            let value_f32: Option<f32> = row.try_get("value").ok();
            let value_str: Option<String> = row.try_get("value").ok();
            let val = value_u32.map(|x| x.to_string()).or(value_f32.map(|x| x.to_string())).or(value_str);
            let value = val.unwrap_or_default();
            let count: i64 = row.try_get("count").unwrap_or(0);

            values_html.push_str(&format!(
                r#"<a href="/search?{}={}" class="value-item">{} ({})</a>"#,
                property, urlencoding::encode(&value), value, count
            ));
        }
        values_html.push_str("</div>");
        content.push_str(&values_html);
    } else {
        content.push_str(&format!("no values found for {}", property));
    }

    let html = layout_view(Some(&property), &content);
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
