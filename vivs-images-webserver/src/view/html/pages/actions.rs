use std::sync::Arc;

use actix_web::web;
use actix_web::Result;
use actix_web::HttpResponse;

use crate::actions::action_registry::ActionRegistry;
use crate::actions::worker_thread::WorkerThread;
use crate::view::html::common::create_html_table;
use crate::view::html::common::encode_string;
use crate::view::html::common::link_html;
use crate::view::html::layout::layout_view;

pub fn action_href(name: String, inner_content: String) -> String {
    link_html("/actions/".to_string() + &encode_string(&name), &inner_content)
}

pub fn task_link_html(task_id: u32, inner_content: String) -> String {
    link_html(format!("/actions/task/{}", task_id), &inner_content)
}

pub async fn view_page_actions(
    actions: web::Data<ActionRegistry>,
    worker: web::Data<Arc<WorkerThread>>,
) -> Result<HttpResponse> {
    let actions_table_html = gen_actions_table_html(&actions);
    let tasks_table_html = gen_tasks_table_html(&worker);
    let content = actions_table_html + &tasks_table_html;
    let html = layout_view(Some("Actions"), &content);
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

fn gen_tasks_table_html(worker: &WorkerThread) -> String {
    let mut tasks = worker.task_manager.get_tasks();
    tasks.sort_by_key(|x| x.action_task_id);
    tasks.reverse();
    
    let rows_html: Vec<String> = tasks.iter().map(|r| {
        let task_id = r.action_task_id;
        format!(r#"<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>"#,
            task_link_html(task_id, r.action_name.clone()),
            task_link_html(task_id, format!("{:?}", r.time_started)),
            task_link_html(task_id, format!("{:?}", r.time_ended)),
            task_link_html(task_id, format!("{:?}", r.completion_status)),
            task_link_html(task_id, format!("{:?}", r.progress))
        )
    }).collect();

    let headers = ["Name".to_string(), "Time Started".to_string(), "Time Ended".to_string(), "Completion Status".to_string(), "Progress".to_string()];
    create_html_table("Tasks", &headers.to_vec(), &rows_html.join(""))
}

fn gen_actions_table_html(actions: &ActionRegistry) -> String {
    let actions = actions.get_all_actions();
    let rows_html: Vec<String> = actions.iter().map(|r| {
        format!(r#"<tr><td>{}</td><td>{}</td></tr>"#, action_href(r.get_name(), r.get_label()), action_href(r.get_name(), r.get_description()))
    }).collect();
    
    let headers = ["Name".to_string(), "Description".to_string()];
    create_html_table("Actions", &headers.to_vec(), &rows_html.join(""))
}