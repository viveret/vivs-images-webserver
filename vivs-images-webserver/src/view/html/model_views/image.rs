use std::path::Path;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use image::DynamicImage;
use sqlx::SqlitePool;

use crate::converters::extract_image_thumbnail::extract_multiple_image_thumbnails_standard_sizes_to_png_vec_u8;
use crate::database::query::query_image_thumbnail::query_thumbnail_table;
use crate::models::image_thumbnail::ImageThumbnail;
use crate::view::html::common::{encode_string, image_html, image_thumbnail_html, link_html};
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
                "thumbnail" => {
                    if let Some(thumb) = &image.thumbnail {
                        format!(r#"<td>{}</td>"#, link_html(view_image_href.clone(), &image_thumbnail_html(&thumb, Some(200))))
                    } else {
                        format!(r#"<td>{}</td>"#, link_html(view_image_href.clone(), &image_html(&image.path, Some(200))))                        
                    }
                },
                "path" => format!(r#"<td>{}</td>"#, link_html(view_image_href.clone(), &image.path)),
                _ => format!(r#"<td>{}</td>"#, v.unwrap_or_default()),
            }
        }).collect::<Vec<String>>().join("");
        let row_html = format!(r#"<tr>{}</tr>"#, row_tds_html);
        html.push_str(&row_html);
    }

    html
}

pub fn generate_image_thumbnail_table(img: &DynamicImage) -> String {
    let imgs_html = match extract_multiple_image_thumbnails_standard_sizes_to_png_vec_u8(img) {
        Ok(thumbs) => {
            let mut imgs = String::new();
            for data in thumbs {
                let s = format!("<img src=\"data:image/png;base64,{}\"/>", BASE64_STANDARD.encode(&data));
                imgs.push_str(&s);
            }
            imgs
        },
        Err(e) => format!("<div>error generating thumbnails: {}</div>", e),
    };
    
    let mut html = String::new();
    let title = "Thumbnails";
    html.push_str(&format!("<h3>{}</h3><div>{}</div>", title, imgs_html));

    html
}

pub fn generate_image_thumbnail_table_thumbs(thumbs: Vec<ImageThumbnail>) -> String {
    let mut imgs = String::new();
    for data in thumbs {
        let s = image_thumbnail_html(&data, None);
        imgs.push_str(&s);
    }
    
    let mut html = String::new();
    let title = "Thumbnails";
    html.push_str(&format!("<h3>{}</h3><div>{}</div>", title, imgs));

    html
}

pub fn generate_image_thumbnail_table_open_img(path: &str) -> String {
    match image::open(Path::new(path)) {
        Ok(img) => generate_image_thumbnail_table(&img),
        Err(e) => format!("<div>{}</div>", e),
    }
}

pub async fn generate_image_thumbnail_table_query_thumbnails_db(path: &str, pool: &SqlitePool) -> String {
    let results = query_thumbnail_table(path, pool).await;
    match results {
        Ok(imgs) => generate_image_thumbnail_table_thumbs(imgs),
        Err(e) => format!("<div>{}</div>", e),
    }
}