use std::io::ErrorKind;

use crossbeam_channel::{Sender, Receiver};

use crate::actions::refresh::analysis_task_item_processor::TaskOrchestrationOptions;

#[derive(Debug, Clone, PartialEq)]
pub enum TaskCompletionStatus {
    NotCompleted,
    Success,
    Failure(String),
}

#[derive(Debug)]
pub enum WorkerToMainMessage {
    TaskStarted(u32),
    TaskCompleted(u32),
    TaskLogInfo(u32, String),
    TaskLogError(u32, String),
    TaskProgressUpdate(u32, f32),
    TaskError(u32, String),
    
    WorkerStarted(String),
    WorkerCompleted,
    WorkerError(String),
}

#[derive(Debug)]
pub enum MainToWorkerMessage {
    StartAction {
        action_name: String,
        dry_run: bool,
        orch_options: TaskOrchestrationOptions,
        task_id: u32,
    },
    Shutdown,
}

#[derive(Clone, Debug)]
pub enum TaskToWorkerMessage {
    Started(u32),
    LogInfo(u32, String),
    LogError(u32, String),
    ProgressUpdate(u32, f32),
    Completed(u32, TaskCompletionStatus),
    Error(u32, String),
}

pub type WorkerToMainSender = Sender<WorkerToMainMessage>;
pub type WorkerToMainReceiver = Receiver<WorkerToMainMessage>;
pub type MainToWorkerSender = Sender<MainToWorkerMessage>;
pub type MainToWorkerReceiver = Receiver<MainToWorkerMessage>;
pub type TaskToWorkerSender = Sender<TaskToWorkerMessage>;
pub type TaskToWorkerReceiver = Receiver<TaskToWorkerMessage>;


pub fn task_to_worker_send_helper(send: &TaskToWorkerSender, msg: TaskToWorkerMessage) -> actix_web::Result<()> {
    send.send(msg)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("{}", e)))
}

pub fn worker_to_main_send_helper(send: &WorkerToMainSender, msg: WorkerToMainMessage) -> actix_web::Result<()> {
    send.send(msg)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("{}", e)))
}

pub fn main_to_worker_send_helper(send: &MainToWorkerSender, msg: MainToWorkerMessage) -> actix_web::Result<()> {
    send.send(msg)
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("{}", e)))
}

            



pub fn task_to_worker_send_helper2(send: &TaskToWorkerSender, msg: TaskToWorkerMessage) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
    send.send(msg)
        .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
}

pub fn worker_to_main_send_helper2(send: &WorkerToMainSender, msg: WorkerToMainMessage) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
    send.send(msg)
        .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
}

pub fn main_to_worker_send_helper2(send: &MainToWorkerSender, msg: MainToWorkerMessage) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
    send.send(msg)
        .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
}