use std::path::Path;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use image::DynamicImage;

use crate::converters::extract_image_thumbnail::{convert_image_to_thumbnail, convert_image_to_thumbnail_image, convert_image_to_vec_u8, extract_multiple_image_thumbnails, ExtractImageThumbnailOptions};
use crate::models::image_thumbnail::ThumbnailFormat;
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

pub fn generate_image_thumbnail_table(img: &DynamicImage) -> String {
    let filter = image::imageops::FilterType::Lanczos3;
    let mut size_list = [8, 16, 32, 64, 128];
    let imgs_html = match extract_multiple_image_thumbnails(&size_list, img, filter) {
        Ok(thumbs) => {
            let mut imgs = String::new();
            for img in thumbs {
                let options = ExtractImageThumbnailOptions {
                    filter,
                    width_and_length: img.width(),
                    output_format: ThumbnailFormat::PNG
                };
                let s = match convert_image_to_vec_u8(&img, &options) {
                    Ok(data) => format!("<img src=\"data:image/png;base64,{}\"/>", BASE64_STANDARD.encode(&data)),
                    Err(e) => format!("<div>error generating {}x{} thumbnail: {}</div>",
                        options.width_and_length, options.width_and_length, e),
                };
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

pub fn generate_image_thumbnail_table_open_img(path: &str) -> String {
    match image::open(Path::new(path)) {
        Ok(img) => generate_image_thumbnail_table(&img),
        Err(e) => format!("<div>{}</div>", e),
    }
}