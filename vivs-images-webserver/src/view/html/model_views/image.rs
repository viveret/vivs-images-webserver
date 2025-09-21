use crate::view::html::common::{encode_string, image_html, link_html};
use crate::models::image::Image;


// Helper function to generate table rows for image listings
pub fn generate_image_table_rows(rows: &[Image], columns: &[String]) -> String {
    let mut html = String::new();
    // println!("Generating table rows for {} images, columns = {:?}", rows.len(), columns);

    for image in rows {
        let view_image_href = "/image".to_string() + "?image_path=" + &encode_string(&image.path);
        let row_tds_html = columns.into_iter().map(|c| {
            let v = image.get_field(c);
            // println!("Column: {}, Value: {:?}", c, v);
            match c.as_ref() {
                "thumbnail" => format!(r#"<td>{}</td>"#, link_html(view_image_href.clone(), &image_html(&image.path, Some(200)))),
                "path" => format!(r#"<td>{}</td>"#, link_html(view_image_href.clone(), &image.path)),
                _ => format!(r#"<td>{}</td>"#, v.unwrap_or_default()),
            }
        }).collect::<Vec<String>>().join("");
        let row_html = format!(r#"<tr>{}</tr>"#, row_tds_html);
        html.push_str(&row_html);
    }

    html
}