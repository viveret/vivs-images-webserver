use std::collections::HashMap;

use actix_web::{web, HttpResponse, Result};
use htmlentity::entity::ICodedDataTrait;

use crate::core::data_context::WebServerActionDataContext;
use crate::database::query::search::{find_image_by_path, SearchBuilder};
use crate::models::image_similarity::ImageSimilarity;
use crate::models::query_params::search_params::SearchParams;
use crate::models::query_params::similar_images_params::SimilarImagesParams;
use crate::view::html::common::{create_html_table, image_html};
use crate::view::html::layout::layout_view;
use crate::view::html::model_views::image::{generate_image_table_rows, generate_image_thumbnail_table_query_thumbnails_db};


pub async fn view_image(
    pool: web::Data<WebServerActionDataContext>,
    params: web::Query<SimilarImagesParams>,
) -> Result<HttpResponse> {
    match find_image_by_path(pool.get_ref().clone(), &params.image_path).await {
        Ok(Some(image)) => {
            let thumbnails_html = generate_image_thumbnail_table_query_thumbnails_db(&params.image_path, &pool.get_ref().pool).await;
            
            let threshold = params.threshold.unwrap_or(1.0);

            let select_from_source_query = SearchBuilder::new()
                .with_base_table("image_similarity")
                .with_select_columns(vec![
                    "CASE WHEN image_similarity.image_path_a = ? THEN image_similarity.image_path_b ELSE image_similarity.image_path_a END as image_path".to_string(),
                ])
                .with_field_meta_columns(ImageSimilarity::get_meta_for_single())
                .with_select_clause_param(params.image_path.clone())
                .with_criteria(vec![
                    ("AND".to_string(), {
                        let mut map = HashMap::new();
                        map.insert("image_similarity.similarity_value >= ?".to_string(), "0.5".to_string());
                        map.insert("image_similarity.similarity_value <= ?".to_string(), threshold.to_string());
                        map
                    }),
                    ("OR".to_string(), {
                        let mut map = HashMap::new();
                        map.insert("image_similarity.image_path_a = ?".to_string(), params.image_path.clone());
                        map.insert("image_similarity.image_path_b = ?".to_string(), params.image_path.clone());
                        map
                    })
                ])
                .with_order_by("image_similarity.similarity_value DESC")
                .with_pagination(20, 0);
            
            let similar_images = SearchBuilder::new()
                .with_base_table("image_similarity")
                .with_base_table_as_select(select_from_source_query)
                .with_select_columns(vec![
                    "image_similarity.image_path".to_string(),
                ])
                .with_default_tables()
                .execute_get_images(pool.as_ref().clone())
                .await;

            let similarity_table_html = match similar_images {
                Ok(images) => {
                    let columns = ["thumbnail", "similarity_value", "path", "camera_model", "lens_model", "exposure_time", "focal_length"];
                    let columns = columns.map(String::from).to_vec();
                    let rows_html = generate_image_table_rows(&images, &columns);
                    create_html_table(
                        &format!("Images similar to {} (threshold: {})", params.image_path, threshold),
                        &SearchParams::get_column_titles(&columns),
                        &rows_html
                    )
                }
                Err(e) => {
                    format!("could not get similarity: {}", e)
                }
            };

            let ocr_text = image.ocr_text.map(|x| x.ocr_text).unwrap_or_default();
            let ocr_text = htmlentity::entity::encode(
                ocr_text.as_bytes(),
                &htmlentity::entity::EncodeType::NamedOrHex,
                &htmlentity::entity::CharacterSet::HtmlAndNonASCII,
            ).to_string().unwrap_or_default();
            let ocr_text = format!("<h4>ocr text:</h4><p><textarea>{}</textarea></p><p>{}</p>", ocr_text, ocr_text);
            let aspect_ratio_html = format!("<p>aspect ratio: {}</p>", image.aspect_ratio.map(|x| x.to_string()).unwrap_or_default());

            let body_html = format!("{}{}{}<h4>other properties:</h4>{}{}", 
                image_html(&params.image_path, Some(200)),
                ocr_text,
                thumbnails_html,
                aspect_ratio_html,
                similarity_table_html
            );

            let html = layout_view(Some("Image Details"), &body_html);
            Ok(HttpResponse::Ok().content_type("text/html").body(html))
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().body(format!("Image {} not found", params.image_path)))
        }
        Err(e) => {
            Ok(HttpResponse::NotFound().body(format!("Image {} not found ({})", params.image_path, e)))
        }
    }
}