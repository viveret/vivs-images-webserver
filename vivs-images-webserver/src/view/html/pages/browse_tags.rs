use actix_web::{web, HttpResponse, Result};
use sqlx::SqlitePool;

use crate::core::data_context::WebServerActionDataContext;
use crate::database::query::query_image_tag::{query_all_tags, query_tag_metrics};
use crate::view::html::layout::layout_view;


async fn generate_tag_interface(pool: &SqlitePool) -> String {
    let mut html = String::from(r#"
    <div class="tags">
        <h2>Browse by Tag</h2>
    "#);

    match query_all_tags(pool).await {
        Ok(all_tags) => {
            html.push_str(r#"<div class="value-list">"#);
            for tag in all_tags {
                let label = if tag.tag_label.is_empty() {
                    &tag.tag_name
                } else {
                    &tag.tag_label
                };
                let desc = if !tag.tag_description.is_empty() {
                    &format!(" - {}", tag.tag_description)
                } else {
                    &String::default()
                };
                html.push_str(&format!(
                    r#"<a href="/browse/tags/{}" class="value-item">{}</a>{}"#,
                    urlencoding::encode(&tag.tag_name), label, desc
                ));
            }
            html.push_str(r#"</div>"#);
        }
        Err(e) => {
            html.push_str(&format!("error: {}", e));
        }
    }

    html.push_str("</div>");
    html
}


pub async fn view_page_tags(
    pool: web::Data<WebServerActionDataContext>,
) -> Result<HttpResponse> {
    let tag_interface = generate_tag_interface(&pool.pool).await;
    let html = layout_view(Some("Browse by Tag"), &tag_interface);
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}


pub async fn view_page_tag_details(
    pool: web::Data<WebServerActionDataContext>,
    tag: web::Path<String>
) -> Result<HttpResponse> {
    if let Some(tag_info) = query_tag_metrics(&tag, &pool.pool).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))? {
        let mut related_tags_html = tag_info.related_tags.iter()
            .map(|t| format!("<li>{} ({})</li>", t.0, t.1))
            .collect::<Vec<String>>()
            .join("");
        if related_tags_html.is_empty() {
            related_tags_html = "no related tags found.".to_string();
        }
        let mut desc = tag_info.tag.tag_description;
        if desc.is_empty() {
            desc = "no description.".to_string();
        }
        
        let content = format!("<p>Description: {}</p><p>Use count: {}</p><p>Related tags: {}</p>", desc, tag_info.use_count, related_tags_html);
        let html = layout_view(Some(&tag_info.tag.tag_name), &content);
        Ok(HttpResponse::Ok().content_type("text/html").body(html))
    } else {
        Ok(HttpResponse::NotFound().content_type("text/html").body(format!("tag {} not found", tag)))
    }
}