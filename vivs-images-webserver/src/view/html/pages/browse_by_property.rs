use actix_web::{HttpResponse, Result};

use crate::models::image::Image;
use crate::view::html::layout::layout_view;


fn generate_properties_interface() -> String {
    let mut html = String::from(r#"
    <div class="browse-properties">
        <h2>Browse by Property</h2>
        <div class="value-list">
    "#);

    let all_properties = Image::get_meta();

    for prop in all_properties {
        if prop.name == "image_path" || prop.name.starts_with("image_path_") || prop.name.ends_with("_key") {
            continue;
        }
        html.push_str(&format!(
            r#"<a href="/browse/by-property/{}" class="value-item">{}</a>"#,
            prop.name, prop.label
        ));
    }

    html.push_str("</div></div>");
    html
}


pub async fn view_page_browse_properties() -> Result<HttpResponse> {
    let properties_interface = generate_properties_interface();
    let html = layout_view(Some("Browse by Property"), &properties_interface);
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}