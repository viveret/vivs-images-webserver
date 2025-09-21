use actix_web::Result;
use sqlx::SqlitePool;

use crate::models::query_params::search_params::SearchParams;
use crate::view::html::model_views::search_params::use_simple_view_checkbox;
use crate::view::html::model_views::search_params::query_string_input;


pub fn gen_regular_search_html(params: &SearchParams) -> String {
    params.fields
        .iter()
        .filter(|f| f.field_meta.is_regular && !f.field_meta.is_advanced)
        .map(|field| format!(r#"<div class="regular-form-group-item">{}</div>"#, field.to_html()))
        .collect::<Vec<String>>()
        .join("\n")
}


pub async fn search_images_simple_form(
    pool: &SqlitePool,
    params: &SearchParams,
) -> Result<String> {
    let html = format!(r#"
    <div class="search-form">
        <h3>Simple Search</h3>
        <form method="get" action="/search">
            <div class="form-group">
                {}
                <button type="submit">Search</button>
                {}
            </div>
            <div class="form-regular-options-group">
                {}
            </div>
        </form>
    </div>
    "#, query_string_input(params.get_query().as_ref().unwrap_or(&String::new())), use_simple_view_checkbox(true), gen_regular_search_html(params));
    Ok(html)
}