use std::sync::Arc;

use actix_web::http::header::LOCATION;
use actix_web::web;
use actix_web::Result;
use actix_web::HttpResponse;

use crate::actions::action_registry::find_action;
use crate::actions::worker_thread::WorkerThread;
use crate::view::html::layout::layout_view;


pub fn submit_action_form(name:& String, label: &String) -> String {
    format!(r#"<form method="POST" action="/actions/start/{}"><button type="submit">{}</button></form>"#, name, label)
}

pub async fn view_page_action_detail_get(
    action_name: web::Path<String>,
) -> Result<HttpResponse> {
    if let Some(action) = find_action(action_name.to_string()) {
        let action_title = action.get_label();
        let content = format!("{} {}", action.get_description(), submit_action_form(&action_name, &action_title));
        let html = layout_view(Some(&action_title), &content);
        Ok(HttpResponse::Ok().content_type("text/html").body(html))
    } else {
        Ok(HttpResponse::NotFound().body(format!("Action {} not found", action_name)))
    }
}

pub async fn view_page_action_detail_post(
    worker_thread_pool: web::Data<Arc<WorkerThread>>,
    action_name: web::Path<String>,
) -> Result<HttpResponse> {
    match worker_thread_pool.get_ref().run_action(action_name.to_string()) {
        Ok(task_id) => {
            let href = format!("/actions/task/{}", task_id);
            Ok(HttpResponse::SeeOther().insert_header((LOCATION, href)).finish())
        }
        Err(e) => {
            println!("error: {}", e);
            Ok(HttpResponse::BadRequest().finish())
        }
    }
}