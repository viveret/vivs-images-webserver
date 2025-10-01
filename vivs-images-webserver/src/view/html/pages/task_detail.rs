use std::sync::Arc;

use actix_web::http::header::LOCATION;
use sqlx::SqlitePool;
use actix_web::web;
use actix_web::Result;
use actix_web::HttpResponse;

use crate::actions::worker_thread::WorkerThread;
use crate::view::html::common::create_html_table;
use crate::view::html::layout::layout_view;


pub fn submit_action_form(name:& String, label: &String) -> String {
    format!(r#"<form method="POST" action="/actions/start/{}"><button type="submit">{}</button></form>"#, name, label)
}

pub async fn view_page_task_detail_get(
    pool: web::Data<SqlitePool>,
    worker_thread_pool: web::Data<Arc<WorkerThread>>,
    task_id: web::Path<String>,
) -> Result<HttpResponse> {
    match task_id.parse() {
        Ok(task_id) => {
            if let Some(task) = worker_thread_pool.get_task(task_id) {
                let task_output = task.get_output();
                let task_output_error = task.get_error_output();
                let action = task.action;
                let action_title = action.get_label();
                let progress_span = format!("<p><h4>Progress: {:.5}%</h4></p>", task.progress * 100.0);
                let rows_html = format!(r#"<td><p>{}</p></td><td><p>{}</p></td>"#, task_output, task_output_error);
                let table_html = create_html_table("Output", &vec!["Standard".to_string(), "Error".to_string()], &rows_html);
                let content = format!("<p>{}</p>{}{}", action.get_description(), progress_span, table_html);
                let html = layout_view(Some(&action_title), &content);
                Ok(HttpResponse::Ok().content_type("text/html").body(html))
            } else {
                Ok(HttpResponse::NotFound().body(format!("Action task {} not found", task_id)))
            }
        },
        Err(e) => {
            Ok(HttpResponse::NotFound().body(format!("Action task {} not found ({})", task_id, e)))
        }
    }
}

pub async fn view_page_task_detail_post(
    pool: web::Data<SqlitePool>,
    action_name: web::Path<String>,
) -> Result<HttpResponse> {
    // if let Some(action) = find_action(action_name.to_string()) {
        // let task_id = worker_thread_pool.get_ref().run_action_in_bg(action_name.to_string())?;
        // let href = format!("/actions/task/{}", task_id);
        // Ok(HttpResponse::SeeOther().insert_header((LOCATION, href)).finish())
    // } else {
        Ok(HttpResponse::NotFound().body(format!("Action {} not found", action_name)))
    // }
}