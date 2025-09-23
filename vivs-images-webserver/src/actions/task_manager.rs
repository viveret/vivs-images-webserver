use std::sync::{Arc, Mutex};
use std::time::Instant;

use chrono::{DateTime, Utc};

use super::channels::TaskCompletionStatus;
use super::action_registry::IWebServerAction;



#[derive(Clone)]
pub struct WebServerActionTask {
    pub action_task_id: u32,
    pub action_name: String,
    pub time_started: DateTime<Utc>,
    pub time_ended: Option<DateTime<Utc>>,
    pub completion_status: TaskCompletionStatus,
    pub progress: f32,
    pub handle: Arc<Mutex<Option<std::thread::JoinHandle<()>>>>,
    pub action: Arc<dyn IWebServerAction>,
    pub output: Arc<Mutex<String>>,
    pub output_error: Arc<Mutex<String>>,
}

impl WebServerActionTask {
    pub fn new(
        action_task_id: u32,
        action_name: String,
        action: Arc<dyn IWebServerAction>,
    ) -> Self {
        Self {
            action_task_id,
            action_name,
            time_started: Utc::now().into(),
            time_ended: None,
            completion_status: TaskCompletionStatus::NotCompleted,
            progress: 0.0,
            handle: Arc::new(Mutex::new(None)),
            action,
            output: Arc::new(Mutex::new(String::new())),
            output_error: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn mark_completed(&mut self, status: TaskCompletionStatus) {
        self.time_ended = Some(Utc::now().into());
        self.completion_status = status;
        self.progress = 1.0;
    }

    pub fn update_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    pub fn assign_thread_handle(&self, handle: std::thread::JoinHandle<()>) {
        let mut h = self.handle.lock().unwrap();
        *h = Some(handle);
    }

    pub fn get_output(&self) -> String {
        self.output.lock().unwrap().clone()
    }

    pub fn get_error_output(&self) -> String {
        self.output_error.lock().unwrap().clone()
    }

    fn append_output(&self, message: &str) {
        let mut output = self.output.lock().unwrap();
        output.push_str(message);
        output.push_str("\n");
    }

    fn append_error_output(&self, message: &str) {
        let mut output = self.output_error.lock().unwrap();
        output.push_str(message);
        output.push_str("\n");
    }
}

#[derive(Clone)]
pub struct TaskManager {
    next_task_id: Arc<Mutex<u32>>,
    active_tasks: Arc<Mutex<std::collections::HashMap<u32, WebServerActionTask>>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            next_task_id: Arc::new(Mutex::new(1)),
            active_tasks: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    pub fn create_task(
        &self,
        action: Arc<dyn IWebServerAction>,
    ) -> u32 {
        let mut next_task_id = self.next_task_id.lock().unwrap();
        let task_id = *next_task_id;
        *next_task_id += 1;

        let task = WebServerActionTask::new(task_id, action.get_name(), action);
        self.active_tasks.lock().unwrap().insert(task_id, task);

        task_id
    }

    pub fn get_tasks(&self) -> Vec<WebServerActionTask> {
        self.active_tasks
            .lock()
            .unwrap()
            .values()
            .into_iter()
            .cloned()
            .collect()
    }

    pub fn get_task(&self, task_id: u32) -> Option<WebServerActionTask> {
        self.active_tasks
            .lock()
            .unwrap()
            .get(&task_id)
            .cloned()
    }

    pub fn update_task_progress(&self, task_id: u32, progress: f32) -> bool {
        if let Some(task) = self.active_tasks.lock().unwrap().get_mut(&task_id) {
            task.update_progress(progress);
            true
        } else {
            false
        }
    }

    pub fn append_task_output(&self, task_id: u32, message: &String) {
        if let Some(task) = self.active_tasks.lock().unwrap().get_mut(&task_id) {
            task.append_output(message);
        }
    }

    pub fn append_task_error_output(&self, task_id: u32, message: &String) {
        if let Some(task) = self.active_tasks.lock().unwrap().get_mut(&task_id) {
            task.append_error_output(message);
        }
    }

    pub fn complete_task(
        &self,
        task_id: u32,
        status: TaskCompletionStatus,
    ) -> bool {
        if let Some(task) = self.active_tasks.lock().unwrap().get_mut(&task_id) {
            task.mark_completed(status);
            true
        } else {
            false
        }
    }

    pub fn remove_task(&self, task_id: u32) -> Option<WebServerActionTask> {
        self.active_tasks.lock().unwrap().remove(&task_id)
    }
    
    pub fn is_task_running(&self, task_id: u32) -> bool {
        self.get_task(task_id)
            .map(|x| x.completion_status == TaskCompletionStatus::NotCompleted)
            .unwrap_or_default()
    }
}