use crossbeam_channel::{select, Receiver};
use sqlx::{Pool, Sqlite};
use tokio::runtime::Runtime;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::actions::action_registry::{ActionRegistry, IWebServerAction};
use crate::actions::channels::{task_to_worker_send_helper, worker_to_main_send_helper, WorkerToMainSender};
use crate::actions::channels::WorkerToMainReceiver;
use crate::actions::channels::WorkerToMainMessage;
use crate::actions::channels::TaskToWorkerSender;
use crate::actions::channels::TaskToWorkerReceiver;
use crate::actions::channels::TaskToWorkerMessage;
use crate::actions::channels::TaskCompletionStatus;
use crate::actions::channels::MainToWorkerMessage;
use crate::actions::channels::MainToWorkerSender;
use crate::actions::task_manager::{TaskManager, WebServerActionTask};
use crate::actions::{task_manager, thread_pool};

#[derive(Clone)]
pub struct WorkerThread {
    pool: Pool<Sqlite>,
    tx_to_main: WorkerToMainSender,
    tx_to_worker: MainToWorkerSender,
    thread_pool: Arc<thread_pool::ThreadPool>,
    pub task_manager: task_manager::TaskManager,
    pub action_registry: ActionRegistry,
    handle: Arc<Mutex<Option<thread::JoinHandle<()>>>>,
}

impl WorkerThread {
    pub fn new(pool: Pool<Sqlite>, tx_to_main: WorkerToMainSender, tx_to_worker: MainToWorkerSender) -> Self {
        let thread_pool = Arc::new(thread_pool::ThreadPool::new(8)); // 8 worker threads
        let task_manager = task_manager::TaskManager::new();
        let action_registry = ActionRegistry::new();

        Self {
            pool,
            tx_to_main,
            tx_to_worker,
            thread_pool,
            task_manager,
            action_registry,
            handle: Arc::new(Mutex::new(None)),
        }
    }

    pub fn spawn(pool: Pool<Sqlite>) -> Arc<Self> {
        let (tx_to_main, rx_from_worker) = crossbeam_channel::unbounded();
        let (tx_to_worker, rx_from_main) = crossbeam_channel::unbounded();

        let worker = Arc::new(Self::new(pool, tx_to_main, tx_to_worker));
        let worker_clone = worker.clone();

        // Spawn main worker thread
        let handle = thread::spawn(move || {
            worker_clone.run(rx_from_main);
        });

        worker.assign_thread_handle(handle);

        // Spawn response handler thread
        let worker_clone2 = worker.clone();
        thread::spawn(move || {
            WorkerThread::handle_responses_from_worker(worker_clone2, rx_from_worker);
        });

        worker
    }

    fn assign_thread_handle(&self, handle: thread::JoinHandle<()>) {
        let mut h = self.handle.lock().unwrap();
        *h = Some(handle);
    }

    fn handle_responses_from_worker(_worker: Arc<WorkerThread>, rx_from_worker: WorkerToMainReceiver) {
        loop {
            match rx_from_worker.recv() {
                Ok(response) => {
                    println!("Worker response: {:?}", response);
                    match response {
                        WorkerToMainMessage::TaskStarted(_task_id) => {},
                        WorkerToMainMessage::TaskCompleted(_task_id) => {},
                        WorkerToMainMessage::TaskProgressUpdate(_task_id, _progress) => {},
                        WorkerToMainMessage::TaskError(_task_id, _e) => {
                            break;
                        },
                        WorkerToMainMessage::TaskLogInfo(_, _) => {},
                        WorkerToMainMessage::TaskLogError(_, _) => {},
                        WorkerToMainMessage::WorkerStarted(_) => {},
                        WorkerToMainMessage::WorkerCompleted => {},
                        WorkerToMainMessage::WorkerError(_) => {},
                    }
                }
                Err(e) => {
                    println!("Worker error: {}", e);
                    break
                }
            }
        }
    }

    fn handle_responses_from_task(
        task_id: u32,
        task_manager: TaskManager,
        rx_from_task: TaskToWorkerReceiver,
        tx_to_main: WorkerToMainSender
    ) -> actix_web::Result<()> {
        loop {
            select! {
                recv(rx_from_task) -> msg => {
                    match msg {
                        Ok(x) => {
                            match x {
                                TaskToWorkerMessage::Started(task_id) => {
                                    task_manager.append_task_output(task_id, &format!("task started {}", task_id));
                                    worker_to_main_send_helper(&tx_to_main, WorkerToMainMessage::TaskStarted(task_id))?;
                                }
                                TaskToWorkerMessage::LogInfo(task_id, message) => {
                                    task_manager.append_task_output(task_id, &message);
                                    worker_to_main_send_helper(&tx_to_main, WorkerToMainMessage::TaskLogInfo(task_id, message))?;
                                }
                                TaskToWorkerMessage::LogError(task_id, message) => {
                                    task_manager.append_task_error_output(task_id, &message);
                                    worker_to_main_send_helper(&tx_to_main, WorkerToMainMessage::TaskLogError(task_id, message))?;
                                }
                                TaskToWorkerMessage::ProgressUpdate(task_id, progress) => {
                                    task_manager.append_task_output(task_id, &format!("task progress {}: {}", task_id, progress));
                                    task_manager.update_task_progress(task_id, progress);
                                    worker_to_main_send_helper(&tx_to_main, WorkerToMainMessage::TaskProgressUpdate(task_id, progress))?;
                                }
                                TaskToWorkerMessage::Completed(task_id, status) => {
                                    task_manager.append_task_output(task_id, &format!("task completed {}: {:?}", task_id, status));
                                    worker_to_main_send_helper(&tx_to_main, WorkerToMainMessage::TaskCompleted(task_id))?;
                                    break;
                                }
                                TaskToWorkerMessage::Error(task_id, error) => {
                                    let msg = format!("task error {}: {}", task_id, error);
                                    task_manager.append_task_output(task_id, &msg);
                                    worker_to_main_send_helper(&tx_to_main, WorkerToMainMessage::TaskError(task_id, msg))?;
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            let msg = format!("task error {}: {}", task_id, e);
                            task_manager.append_task_output(task_id, &msg);
                            worker_to_main_send_helper(&tx_to_main, WorkerToMainMessage::TaskError(task_id, msg))?;
                            break;
                        }
                    }
                }
                default(Duration::from_millis(100)) => {
                    // Check for task updates or other periodic tasks
                }
            }
        }
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.handle
            .lock()
            .map(|h| h.as_ref().map(|h| !h.is_finished()).unwrap_or(false))
            .unwrap_or(false)
    }

    pub fn get_task(&self, task_id: u32) -> Option<WebServerActionTask> {
        self.task_manager.get_task(task_id)
    }

    pub fn new_task_id_for_action(&self, action: Arc<dyn IWebServerAction>) -> u32 {
        let task_id = self.task_manager.create_task(action);
        task_id
    }

    pub fn run_action(&self, action_name: String, dry_run: bool) -> Result<u32, String> {
        let action = self.action_registry
            .get_action(&action_name)
            .ok_or_else(|| format!("Action {} not found", action_name))?;

        if !action.get_is_runnable() {
            return Err(format!("Action {} is not runnable", action_name));
        }

        if dry_run && !action.get_can_dry_run() {
            return Err(format!("Action {} is not runnable in dry run mode", action_name));
        }

        let task_id = self.new_task_id_for_action(action);
        self.tx_to_worker.send(MainToWorkerMessage::StartAction { action_name, dry_run, task_id })
            .map_err(|e| {
                format!("could not send start action message: {}", e)
            })?;

        Ok(task_id)
    }

    fn execute_task(
        task_id: u32,
        dry_run: bool,
        action: Arc<dyn IWebServerAction>,
        pool: Pool<Sqlite>,
        tx_to_worker: &TaskToWorkerSender,
        task_manager: &task_manager::TaskManager,
    ) -> actix_web::Result<()> {
        // Send start notification
        task_to_worker_send_helper(&tx_to_worker, TaskToWorkerMessage::Started(task_id))?;

        // Update initial progress
        task_manager.update_task_progress(task_id, 0.0);
        task_to_worker_send_helper(&tx_to_worker, TaskToWorkerMessage::ProgressUpdate(task_id, 0.0))?;

        // Execute the action
        let rt  = Runtime::new().unwrap();

        // Spawn the root task
        let tx_to_worker2 = tx_to_worker.clone();
        let result = rt.block_on(async {
            action.run_task(pool, tx_to_worker2, dry_run, task_id).await
        });

        // Handle completion
        let status = match result {
            Ok(()) => TaskCompletionStatus::Success,
            Err(e) => TaskCompletionStatus::Failure(e.to_string()),
        };

        task_manager.complete_task(task_id, status.clone());
        task_to_worker_send_helper(&tx_to_worker, TaskToWorkerMessage::Completed(task_id, status))?;

        Ok(())
    }

    fn run(&self, rx_from_main: Receiver<MainToWorkerMessage>) {
        println!("Worker thread running");
        let rt  = Runtime::new().unwrap();

        // Spawn the root task
        rt.block_on(async {
            while self.is_running() {
                select! {
                    recv(rx_from_main) -> msg => {
                        match msg {
                            Ok(MainToWorkerMessage::StartAction { action_name, dry_run, task_id }) => {
                                self.handle_start_action(action_name, dry_run, task_id);
                            }
                            Ok(MainToWorkerMessage::Shutdown) => {
                                println!("Worker shutdown");
                                break;
                            }
                            Err(e) => {
                                println!("Worker thread error: {}", e);
                                break;
                            }
                        }
                    }
                    default(Duration::from_millis(100)) => {
                        // Check for task updates or other periodic tasks
                    }
                }
            }
        });
    }

    fn handle_start_action(&self, action_name: String, dry_run: bool, task_id: u32) {
        if let Some(action) = self.action_registry.get_action(&action_name) {
            let task_manager = self.task_manager.clone();
            let pool = self.pool.clone();
            let (tx_to_worker, rx_from_task) = crossbeam_channel::unbounded();
            self.thread_pool.execute(move || {
                if let Err(e) = WorkerThread::execute_task(task_id, dry_run, action, pool, &tx_to_worker, &task_manager) {
                    task_manager.append_task_output(task_id, &format!("run action error: {}", e));
                }
            });

            let tx_to_main = self.tx_to_main.clone();
            let task_manager = self.task_manager.clone();
            self.thread_pool.execute(move || {
                WorkerThread::handle_responses_from_task(task_id, task_manager, rx_from_task, tx_to_main)
                    .expect("WorkerThread::handle_responses_from_task cannot fail");
            });
        } else {
            self.tx_to_main.send(WorkerToMainMessage::WorkerError(
                format!("Action {} not found", action_name)
            )).ok();
        }
    }
}

impl Drop for WorkerThread {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.lock().unwrap().take() {
            let _ = handle.join();
        }
    }
}