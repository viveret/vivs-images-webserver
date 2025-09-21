use std::collections::HashMap;

use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Result;
use chrono::DateTime;
use chrono::Utc;
use sqlx::SqlitePool;

use crate::actions::common::get_all_action_indicators;
use crate::database::query::query_top_level_metrics::get_top_level_metrics;
use crate::models::query_params::search_params::SearchParams;
use crate::view::html::common::link_html;
use crate::view::html::layout::layout_view;
use crate::view::html::model_views::search_params_simple::search_images_simple_form;
use crate::view::html::pages::actions::action_href;


pub async fn get_indicators_html(pool: &SqlitePool) -> Result<String> {
    let indicators_to_list = get_all_action_indicators();
    let mut values_of_indicators = HashMap::new();
    for indicator in indicators_to_list.iter() {
        let v = indicator.perform_indicator_check_action(pool).await?;
        values_of_indicators.insert(indicator.get_name(), v);
    }
    
    let indicators = indicators_to_list.iter().filter_map(|i| {
        values_of_indicators.get(&i.get_name()).cloned().and_then(|v| {
            let link_content = format!(r#"<b>{}</b> - {} <p>{}</p>"#, i.get_label(), i.get_description(), v.1);
            let action_link_html = action_href(i.get_action_name(), link_content);
            Some((v.0, format!(r#"<li>{}</li>"#, action_link_html)))
        })
    }).collect::<Vec<(bool, String)>>();// .join("");

    let activated_html = indicators.iter().filter(|x| x.0).map(|x| x.1.clone()).collect::<Vec<String>>().join("");
    let deactivated_html = indicators.iter().filter(|x| !x.0).map(|x| x.1.clone()).collect::<Vec<String>>().join("");

    Ok(format!("Activated: <ul>{}</ul> Deactivated: <ul>{}</ul>", activated_html, deactivated_html))
}

pub async fn index(
    pool: web::Data<SqlitePool>,
) -> Result<HttpResponse> {
    let mut content = search_images_simple_form(pool.get_ref(), &SearchParams::default()).await?;
    content.push_str(r#"
        <div class="info">
            <h3>Welcome to Viv's Image Search</h3>
            <p>Use the search form above to find images based on your query.</p>
        </div>
    "#);

    let metrics = get_top_level_metrics(&pool).await?;
    let datetime: DateTime<Utc> = metrics.last_updated.into();
    let local_time = datetime.with_timezone(&chrono::Local);
    
    // push some basic info about the app and the dataset
    let dataset_info = format!(r#"
        <div class="dataset-info">
            <h4>Dataset Information</h4>
            <ul>
                <li>Total Images: {}</li>
                <li>Categories: {}</li>
                <li>Last Updated: {}</li>
            </ul>
        </div>
    "#, metrics.total_images, metrics.categories, local_time.format("%B %d, %Y, at %T")); // show pretty date
    content.push_str(&dataset_info);

    let indicators_to_list_html = get_indicators_html(&pool).await?;
    content.push_str(&format!(r#"
        <div class="admin-dashboard">
            <h4>Action Indicators</h4>
            {}
        </div>
    "#, indicators_to_list_html));

    let html = layout_view(Some("Home"), &content);
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}