use actix_web::{web, HttpResponse, Result};
use sqlx::SqlitePool;

use crate::models::query_params::search_params::SearchParams;
use crate::models::query_params::similar_images_params::SimilarImagesParams;
use crate::database::common::execute_query;
use crate::view::html::common::{create_html_table, image_html};
use crate::view::html::layout::layout_view;
use crate::view::html::model_views::image::generate_image_table_rows;


pub async fn view_image(
    pool: web::Data<SqlitePool>,
    params: web::Query<SimilarImagesParams>,
) -> Result<HttpResponse> {
    let threshold = params.threshold.unwrap_or(0.8);

    let rows = execute_query(pool.get_ref(),
        r#"
        SELECT 
            CASE 
                WHEN isim.image_path_a = ? THEN isim.image_path_b 
                ELSE isim.image_path_a 
            END as related_image_path,
            isim.similarity_value,
            ie.*
        FROM image_similarity isim
        LEFT JOIN image_exif ie ON 
            (isim.image_path_a = ? AND ie.image_path = isim.image_path_b)
            OR (isim.image_path_b = ? AND ie.image_path = isim.image_path_a)
        WHERE 
            (isim.image_path_a = ? OR isim.image_path_b = ?)
            AND isim.similarity_value >= ?
        ORDER BY isim.similarity_value ASC
        LIMIT 20
        "#,
        vec![
            &params.image_path,  // 1st param for CASE
            &params.image_path,  // 2nd param for JOIN
            &params.image_path,  // 3rd param for JOIN  
            &params.image_path,  // 4th param for WHERE
            &params.image_path,  // 5th param for WHERE
            &threshold.to_string()  // 6th param for similarity
        ]
    ).await?;

    let rows = rows.into_iter().map(|row| {
        crate::models::image::Image::new(&row)
    }).collect::<Vec<_>>();
    
    let columns = ["thumbnail", "similarity_value", "path", "camera_model", "lens_model", "exposure_time", "iso", "focal_length"];
    let columns = columns.map(String::from).to_vec();
    let rows_html = generate_image_table_rows(&rows, &columns);
    let table_html = create_html_table(
        &format!("Images similar to {}", params.image_path),
        &SearchParams::get_column_titles(&columns),
        &rows_html
    );
    let body_html = image_html(&params.image_path, Some(200)) + &table_html;

    let html = layout_view(Some("Image Details"), &body_html);
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}