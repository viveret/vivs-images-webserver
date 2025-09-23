use std::sync::Arc;

use actix_web::http::header::LOCATION;
use actix_web::web;
use actix_web::HttpRequest;
use actix_web::Result;
use actix_web::HttpResponse;
use serde::Deserialize;

use crate::actions::action_registry::find_action;
use crate::actions::worker_thread::WorkerThread;
use crate::view::html::layout::layout_view;


pub fn submit_action_form(name:& String, label: &String, dry_run: bool) -> String {
    format!(
        r#"<form method="POST" action="/actions/start/{}">
                <button type="submit">{}</button>
                <input type="text" name="dry_run" value="{}" />
            </form>"#, name, label, dry_run)
}

pub async fn view_page_action_detail_get(
    action_name: web::Path<String>,
) -> Result<HttpResponse> {
    if let Some(action) = find_action(action_name.to_string()) {
        let action_title = action.get_label();
        let mut submit_actions = vec![
            submit_action_form(&action_name, &action_title, false)
        ];
        if action.get_can_dry_run() {
            let action_title_dry_run = format!("{} (dry run)", action_title);
            submit_actions.push(submit_action_form(&action_name, &action_title_dry_run, true));
        }
        let submit_actions_html = submit_actions.join("");
        let content = format!("{} {}", action.get_description(), submit_actions_html);
        let html = layout_view(Some(&action_title), &content);
        Ok(HttpResponse::Ok().content_type("text/html").body(html))
    } else {
        Ok(HttpResponse::NotFound().body(format!("Action {} not found", action_name)))
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ActionTaskPostOptions {
    pub dry_run: Option<String>,
}

pub async fn view_page_action_detail_post(
    req: HttpRequest, 
    worker_thread_pool: web::Data<Arc<WorkerThread>>,
    action_name: web::Path<String>,
    web::Form(form): web::Form<ActionTaskPostOptions>,
) -> Result<HttpResponse> {
    match worker_thread_pool.get_ref().run_action(action_name.to_string(), form.dry_run.unwrap_or_default() == "true") {
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