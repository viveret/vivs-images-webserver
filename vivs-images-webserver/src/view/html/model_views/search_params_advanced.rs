use actix_web::Result;
use sqlx::SqlitePool;

use crate::models::query_params::search_params::SearchParams;
use crate::view::html::model_views::search_params::{query_string_input, use_simple_view_checkbox};
use crate::view::html::model_views::search_params_simple::gen_regular_search_html;



pub fn gen_advanced_search_html(params: &SearchParams) -> String {
    params.fields
        .iter()
        .filter(|f| f.field_meta.is_advanced)
        .map(|field| format!(r#"<div class="advanced-form-group-item">{}</div>"#, field.to_html()))
        .collect::<Vec<String>>()
        .join("\n")
}

pub async fn search_images_advanced_form(
    pool: &SqlitePool,
    params: &SearchParams,
) -> Result<String> {
    let regular_search_html = gen_regular_search_html(params);
    let advanced_search_html = gen_advanced_search_html(params);

    let html = format!(r#"
    <div class="search-form">
        <h3>Advanced Search</h3>
        <form method="get" action="/search">
            <div class="form-group">
                {}
                <button type="submit">Search</button>
                {}
            </div>
            <div class="form-regular-options-group">
                {}
            </div>
            <div class="form-advanced-options-group">
                {}
            </div>
        </form>
    </div>
    "#, query_string_input(params.get_query().as_ref().unwrap_or(&String::new())), use_simple_view_checkbox(false), regular_search_html, advanced_search_html);
    Ok(html)
}