use std::collections::HashMap;
use std::path::PathBuf;

use actix_web::{web, HttpResponse, Result};

use crate::core::data_context::WebServerActionDataContext;
use crate::database::query::search::{search_images_by_criteria};
use crate::filesystem::query::images::get_photo_sync_path;
use crate::models::query_params::search_params::SearchParams;
use crate::view::html::common::create_html_table;
use crate::view::html::layout::layout_view;
use crate::view::html::model_views::image::generate_image_table_rows;



fn gen_browse_filesystem_href(path: &String) -> String {
    format!("/browse/filesystem?path={}", urlencoding::encode(path))
}

fn gen_browse_filesystem_html_link(path: &String, content: String) -> String {
    format!("<a href=\"{}\">{}</a>", gen_browse_filesystem_href(path), content)
}


fn generate_browse_filesystem_interface(path: String) -> String {
    let mut html = String::from(r#"<div class="filesystem"><ul>"#);
    let sync_path = get_photo_sync_path().unwrap();
    let mut actual_path = PathBuf::new();
    actual_path.push(&sync_path);

    actual_path.push(
        if path.starts_with("/") {
            &path[1..]
        } else {
            path.as_str()
        }
    );
    
    match std::fs::read_dir(actual_path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let name = entry.file_name().into_string().unwrap();
                        if name.starts_with(".") {
                            continue;
                        }
                        let item_path = entry.path();
                        if item_path.is_dir() {
                            let item_path = item_path.to_str().unwrap()[sync_path.len()..].to_string();
                            html.push_str(&format!(
                                r#"<li><div class="fs-item fs-folder">
                                    {}
                                </div></li>"#,
                                gen_browse_filesystem_html_link(&item_path, name)
                            ));
                        } else {
                            let href = "";
                            html.push_str(&format!(
                                r#"<li><div class="fs-item fs-file">
                                    <a href="{}">{}</a>
                                </div></li>"#,
                                href, name
                            ));
                        }
                    }
                    Err(e) => {
                        html.push_str(&format!("error: {}", e));
                    }
                }
            }
        }
        Err(e) => {
            html.push_str(&format!("error: {}", e));
        }
    }

    html.push_str("</ul></div>");
    html
}


pub async fn view_page_browse_filesystem(
    pool: web::Data<WebServerActionDataContext>,
    query: web::Query<HashMap<String, String>>
) -> Result<HttpResponse> {
    let path = query.0.get("path").cloned().unwrap_or_default();
    let mut params = SearchParams::default();
    params.set_field_value("image_path", Some(path.clone()))?;
    params.set_field_value("limit", Some("100".to_string()))?;

    let image_search = search_images_by_criteria(pool.get_ref().clone(), &params, Some("image_taken_at DESC"))
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let params_title = params.to_string();
    let columns_default = ["thumbnail", "name", "ocr_text", "camera_model", "focal_length", "brightness", "width_pixels", "height_pixels"];
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
    let fs_interface = generate_browse_filesystem_interface(path);
    content_html.push_str(&fs_interface);
    content_html.push_str(&table_html);

    let html = layout_view(Some("Browse Filesystem"), &content_html);
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}