// analysis_task_item_processor.rs

use std::io::ErrorKind;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use convert_case::Case;
use convert_case::Casing;
use nameof::name_of_type;
use async_trait::async_trait;
use crossbeam_channel::{bounded, Sender, Receiver};

use crate::core::data_context::WebServerActionDataContext;
use crate::actions::channels::TaskToWorkerSender;
use crate::actions::channels::TaskToWorkerMessage;
use crate::actions::channels::task_to_worker_send_helper;
use crate::actions::action_registry::IWebServerAction;
use crate::calc::math::calculate_progress;


pub type ProgressListenerFn = Arc<dyn Fn(f32) -> () + Send + Sync + 'static>;
pub type LogInfoListenerFn = Arc<dyn Fn(&str) -> () + Send + Sync + 'static>;
pub type LogProgListenerPair = (ProgressListenerFn, LogInfoListenerFn);


#[async_trait]
pub trait AnalysisTaskItemProcessor<TAnalysis, TTaskItem, TTaskItemList, TTaskOutput>: Send + Sync 
where 
    TTaskItem: Send + Sync + std::fmt::Display + Clone,
    TTaskItemList: Send + Sync + IntoIterator<Item = TTaskItem> + 'static,
    TTaskOutput: Send + Sync + std::fmt::Display,
    TAnalysis: Send + Sync + std::fmt::Display,
{
    async fn get_analysis(&self, pool: WebServerActionDataContext, log_prog_listener: Option<LogProgListenerPair>) -> actix_web::Result<TAnalysis, Box<dyn std::error::Error + Send>>;
    async fn get_task_items_from_analysis(&self, pool: WebServerActionDataContext, analysis: TAnalysis, log_prog_listener: Option<LogProgListenerPair>) -> actix_web::Result<TTaskItemList, Box<dyn std::error::Error + Send>>;
    async fn process_task_item(&self, task_item: TTaskItem, dry_run: bool, pool: WebServerActionDataContext) -> actix_web::Result<Option<TTaskOutput>, Box<dyn std::error::Error + Send>>;
    async fn process_task_output(&self, task_output: TTaskOutput, pool: WebServerActionDataContext) -> actix_web::Result<(), Box<dyn std::error::Error + Send>>;
    async fn task_already_completed(&self, task_input: &TTaskItem, pool: WebServerActionDataContext) -> actix_web::Result<bool, Box<dyn std::error::Error + Send>>;
    fn get_description(&self) -> String;
    fn get_item_name(&self) -> String;
    fn get_process_action_name(&self) -> String;
}

pub struct AnalysisTaskItemProcessorOrchestrator<TAnalysis, TTaskItem, TTaskItemList, TTaskOutput> 
where
    TTaskItem: Send + Sync + std::fmt::Display + Clone,
    TTaskItemList: Send + Sync + IntoIterator<Item = TTaskItem> + 'static,
    TTaskOutput: Send + Sync + std::fmt::Display,
    TAnalysis: Send + Sync + std::fmt::Display,
{
    processor: Arc<dyn AnalysisTaskItemProcessor<TAnalysis, TTaskItem, TTaskItemList, TTaskOutput>>,
}

// Message types for thread communication
enum ThreadMessage<TTaskItem> {
    Task(TTaskItem, usize), // task item and its index for progress tracking
    Shutdown,
}

struct ThreadResult {
    _index: usize,
    _success: bool,
    error_message: Option<String>,
}

impl<TAnalysis, TTaskItem, TTaskItemList, TTaskOutput> AnalysisTaskItemProcessorOrchestrator<TAnalysis, TTaskItem, TTaskItemList, TTaskOutput> 
where
    TTaskItem: Send + Sync + std::fmt::Display + Clone + 'static,
    TTaskItemList: Send + Sync + IntoIterator<Item = TTaskItem> + 'static,
    TTaskOutput: Send + Sync + std::fmt::Display + 'static,
    TAnalysis: Send + Sync + std::fmt::Display + 'static,
{
    pub fn new(processor: Arc<dyn AnalysisTaskItemProcessor<TAnalysis, TTaskItem, TTaskItemList, TTaskOutput>>) -> Self {
        Self {
            processor
        }
    }

    pub async fn process_task_item(
        processor: Arc<dyn AnalysisTaskItemProcessor<TAnalysis, TTaskItem, TTaskItemList, TTaskOutput>>,
        pool: WebServerActionDataContext,
        send: TaskToWorkerSender,
        dry_run: bool,
        task_id: u32,
        task_input: TTaskItem,
    ) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        let task_item_str = format!("{}", task_input);
        match processor.process_task_item(task_input, dry_run, pool.clone()).await {
            Ok(task_output) => {
                if dry_run {
                    if let Some(task_output) = task_output {
                        Self::send_log_info(&send, task_id, 
                            format!("Dry run for {}: {}", task_item_str, task_output))?;
                    }
                } else {
                    if let Some(task_output) = task_output {
                        match processor.process_task_output(task_output, pool.clone()).await {
                            Ok(()) => {
                                Self::send_log_info(&send, task_id, 
                                    format!("{} processed {} successfully", processor.get_item_name(), task_item_str))?;
                            },
                            Err(e) => {
                                Self::send_log_error(&send, task_id, 
                                    format!("{} process {} output error: {}", processor.get_item_name(), task_item_str, e))?;
                            }
                        }
                    } else {
                        // nothing
                    }
                }
            },
            Err(e) => {
                Self::send_log_error(&send, task_id, 
                    format!("{} process {} error: {}", processor.get_item_name(), task_item_str, e))?;
            },
        }
        Ok(())
    }

    // Process all missing similarity tasks with progress tracking
    pub async fn process_tasks_linear(
        &self,
        pool: WebServerActionDataContext,
        send: &TaskToWorkerSender,
        dry_run: bool,
        task_id: u32,
        task_items: TTaskItemList,
    ) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        let tasks_vec: Vec<TTaskItem> = task_items.into_iter().collect();
        let total_tasks = tasks_vec.len();
        
        for (index, task_item) in tasks_vec.into_iter().enumerate() {
            Self::process_task_item(
                self.processor.clone(),
                pool.clone(), 
                send.clone(), 
                dry_run, 
                task_id, 
                task_item
            ).await?;
            
            // Update progress
            Self::send_progress_update(send, task_id, calculate_progress(index, total_tasks))?;
        }
        
        Ok(())
    }

    fn create_parallel_task_processing_thread(
        processor: Arc<dyn AnalysisTaskItemProcessor<TAnalysis, TTaskItem, TTaskItemList, TTaskOutput>>,
        pool: WebServerActionDataContext,
        task_receiver: Receiver<ThreadMessage<TTaskItem>>,
        result_sender: Sender<ThreadResult>,
        dry_run: bool,
        task_id: u32,
        send: TaskToWorkerSender
    ) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            while let Ok(message) = task_receiver.recv() {
                match message {
                    ThreadMessage::Task(task_input, index) => {
                        let processor2 = processor.clone();
                        let pool2 = pool.clone();
                        let send2 = send.clone();
                        let result = rt.block_on(async move {
                            Self::process_task_item(processor2, pool2, send2, dry_run, task_id, task_input).await
                        });
                        
                        match result {
                            Ok(()) => {
                                let _ = result_sender.send(ThreadResult {
                                    _index: index,
                                    _success: true,
                                    error_message: None,
                                });
                            },
                            Err(e) => {
                                let _ = result_sender.send(ThreadResult {
                                    _index: index,
                                    _success: false,
                                    error_message: Some(format!("{}", e)),
                                });
                            },
                        }
                    }
                    ThreadMessage::Shutdown => {
                        break;
                    }
                }
            }
        })
    }

    // Process tasks in parallel with true multi-threading
    pub async fn process_tasks_parallel(
        &self,
        pool: WebServerActionDataContext,
        send: &TaskToWorkerSender,
        dry_run: bool,
        task_id: u32,
        task_items: TTaskItemList,
        orch_options: TaskOrchestrationOptions,
    ) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        let tasks_vec: Vec<TTaskItem> = task_items.into_iter().collect();
        let total_tasks = tasks_vec.len();
        
        // Create channels for task distribution - one channel per worker thread
        let mut task_senders = Vec::new();
        let mut task_receivers = Vec::new();
        for _ in 0..orch_options.max_concurrent {
            let (task_sender, task_receiver) = bounded::<ThreadMessage<TTaskItem>>(orch_options.max_concurrent * 2);
            task_senders.push(task_sender);
            task_receivers.push(task_receiver);
        }
        
        let (result_sender, result_receiver) = bounded::<ThreadResult>(total_tasks);
        
        Self::send_log_info(&send, task_id, format!("Creating {} task threads", orch_options.max_concurrent))?;
        let mut worker_handles: Vec<thread::JoinHandle<()>> = Vec::new();
        for task_receiver in task_receivers.into_iter() {
            let handle = Self::create_parallel_task_processing_thread(
                self.processor.clone(),
                pool.clone(),
                task_receiver,
                result_sender.clone(),
                dry_run,
                task_id,
                send.clone()
            );
            worker_handles.push(handle);
        }
        
        Self::send_log_info(&send, task_id, format!("Checking tasks for validity"))?;

        // Create channels for communication between validation thread and main thread
        let (progress_sender, progress_receiver) = crossbeam_channel::unbounded();

        // Spawn validation thread - this needs to be a separate thread so that we can also distribute valid tasks as they are validated
        let processor = self.processor.clone();
        let send2 = send.clone();
        let tasks_vec2 = tasks_vec.clone();
        let validation_handle = thread::spawn(move || {
            // Use tokio::runtime to run async code in a thread
            let rt = tokio::runtime::Runtime::new().unwrap();
            for (index, task_input) in tasks_vec2.into_iter().enumerate() {
                // Clone what we need for the async block
                let processor = processor.clone();
                let pool = pool.clone();
                let prog_sender = progress_sender.clone();
                let task_id = task_id;
                let send = send2.clone();
                
                rt.block_on(async move {
                    // Check if task already exists in database (async check in validation thread)
                    let should_process = !processor.task_already_completed(&task_input, pool).await.unwrap_or(false);
                    
                    // Send progress update for validation phase
                    if prog_sender.send((index, should_process)).is_err() {
                        let _ = Self::send_log_error(&send, task_id, "Failed to send progress update".to_string());
                    }
                });
            }
        });

        let mut actual_total = 0;
        let mut completed_tasks = 0;
        let mut next_worker_index = 0;

        // Process validation results and distribute tasks to worker threads
        while let Ok((index, should_process)) = progress_receiver.recv() {
            // Update progress for validation phase
            Self::send_progress_update(send, task_id, calculate_progress(index, total_tasks) * 0.5)?; // 50% for validation
            
            if should_process {
                actual_total += 1;
                
                // Use round-robin distribution to balance workload
                let worker_index = next_worker_index % orch_options.max_concurrent;
                next_worker_index += 1;
                
                if let Err(e) = task_senders[worker_index].send(ThreadMessage::Task(tasks_vec[index].clone(), index)) {
                    Self::send_log_error(send, task_id, format!("Failed to send task to worker thread: {}", e))?;
                    break;
                }
            }
        }

        // Wait for validation thread to complete
        validation_handle.join().unwrap();

        // Signal shutdown to all workers
        for task_sender in task_senders {
            let _ = task_sender.send(ThreadMessage::Shutdown);
        }
        
        // Collect results and update progress
        for _ in 0..actual_total {
            if let Ok(result) = result_receiver.recv_timeout(Duration::from_secs(1)) {
                completed_tasks += 1;
                
                // Handle task result
                if let Some(error_msg) = result.error_message {
                    Self::send_log_error(send, task_id, error_msg)?;
                }
                
                // Update progress for processing phase (50-100%)
                let progress = 0.5 + (calculate_progress(completed_tasks, actual_total) * 0.5);
                Self::send_progress_update(send, task_id, progress)?;
            } else {
                println!("waiting for worker results...");
            }
        }
        
        // Wait for all worker threads to finish
        for handle in worker_handles {
            let _ = handle.join();
        }
        
        Ok(())
    }

    // Helper methods for sending messages
    fn send_message(
        send: &TaskToWorkerSender, 
        _task_id: u32, 
        message: TaskToWorkerMessage
    ) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        task_to_worker_send_helper(send, message)
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    fn send_log_info(send: &TaskToWorkerSender, task_id: u32, message: String) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        Self::send_message(send, task_id, TaskToWorkerMessage::LogInfo(task_id, message))
    }

    fn send_log_error(send: &TaskToWorkerSender, task_id: u32, message: String) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        Self::send_message(send, task_id, TaskToWorkerMessage::LogError(task_id, message))
    }

    fn send_progress_update(send: &TaskToWorkerSender, task_id: u32, progress: f32) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        Self::send_message(send, task_id, TaskToWorkerMessage::ProgressUpdate(task_id, progress))
    }

    async fn get_task_items_from_analysis(
        &self, 
        pool: WebServerActionDataContext, 
        analysis: TAnalysis,
        log_prog_listener: Option<LogProgListenerPair>,
    ) -> actix_web::Result<TTaskItemList, Box<dyn std::error::Error + Send>> {
        self.processor.get_task_items_from_analysis(pool, analysis, log_prog_listener).await
    }

    async fn run_task_parallel_option(&self, pool: WebServerActionDataContext, send: TaskToWorkerSender, dry_run: bool, task_id: u32, orch_options: TaskOrchestrationOptions) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        let send2 = send.clone();
        let progress_listener: Arc<dyn Fn(f32) + Send + Sync + 'static> = Arc::new(move |progress| {
            let _ = Self::send_progress_update(&send2, task_id, progress);
        });
        let send2 = send.clone();
        let log_listener: Arc<dyn Fn(&str) + Send + Sync + 'static> = Arc::new(move |msg| {
            let _ = Self::send_log_info(&send2, task_id, msg.to_string());
        });
        let log_prog_listener: Option<LogProgListenerPair> = Some((progress_listener, log_listener));

        Self::send_log_info(&send, task_id, format!("Getting analysis..."))?;
        let analysis = self.processor.get_analysis(pool.clone(), log_prog_listener.clone()).await?;
        Self::send_log_info(&send, task_id, format!("Analysis result:\n{}", format!("{}", analysis)))?;
        
        Self::send_log_info(&send, task_id, format!("Getting task items from analysis..."))?;
        let task_items = self.get_task_items_from_analysis(pool.clone(), analysis, log_prog_listener).await?;

        Self::send_progress_update(&send, task_id, 0.0)?;

        if orch_options.run_in_parallel {
            Self::send_log_info(&send, task_id, format!("Running tasks in parallel"))?;
            self.process_tasks_parallel(
                pool, &send, dry_run, task_id, task_items, orch_options
            ).await?;
        } else {
            Self::send_log_info(&send, task_id, format!("Running tasks linearly"))?;
            self.process_tasks_linear(
                pool, &send, dry_run, task_id, task_items
            ).await?;
        }
        
        Self::send_progress_update(&send, task_id, 1.0)?;
        Ok(())
    }
}

#[async_trait]
impl<TAnalysis, TTaskItem, TTaskItemList, TTaskOutput> IWebServerAction for AnalysisTaskItemProcessorOrchestrator<TAnalysis, TTaskItem, TTaskItemList, TTaskOutput> 
where
    TTaskItem: Send + Sync + std::fmt::Display + Clone + 'static,
    TTaskItemList: Send + Sync + IntoIterator<Item = TTaskItem> + 'static,
    TTaskOutput: Send + Sync + std::fmt::Display + 'static,
    TAnalysis: Send + Sync + std::fmt::Display + 'static,
{
    fn get_name(&self) -> String {
        format!("{}_{}",
            self.processor.get_process_action_name().to_case(Case::Snake),
            self.processor.get_item_name().to_case(Case::Snake)
        )
    }

    fn get_label(&self) -> String {
        format!("{} {} {}",
            name_of_type!(AnalysisTaskItemProcessorOrchestrator<TAnalysis, TTaskItem, TTaskItemList, TTaskOutput>).to_case(Case::Sentence),
            self.processor.get_process_action_name().to_case(Case::Sentence),
            self.processor.get_item_name().to_case(Case::Sentence)
        )
    }

    fn get_description(&self) -> String { self.processor.get_description() }

    fn get_is_runnable(&self) -> bool { true }
    
    fn get_can_dry_run(&self) -> bool { true }
    
    async fn run_task(&self,
        pool: WebServerActionDataContext,
        send: TaskToWorkerSender,
        dry_run: bool,
        task_id: u32,
        orch_options: TaskOrchestrationOptions
    ) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        self.run_task_parallel_option(pool, send, dry_run, task_id, orch_options).await
    }
}

#[derive(Debug)]
pub struct TaskOrchestrationOptions {
    pub run_in_parallel: bool,
    pub max_concurrent: usize,
    pub requests_per_second: f32,
}

impl TaskOrchestrationOptions {
    pub fn new_linear() -> Self {
        Self {
            run_in_parallel: false,
            max_concurrent: 0,
            requests_per_second: 0.0,
        }
    }

    pub fn new(
        max_concurrent: usize,
        requests_per_second: f32,
    ) -> Self {
        Self {
            run_in_parallel: true,
            max_concurrent,
            requests_per_second
        }
    }

    pub fn new_defaults() -> Self { Self::new(8, 16.0) }

    pub fn new_faster() -> Self { Self::new_defaults().mul(2) }

    pub fn new_extreme() -> Self { Self::new_defaults().mul(8) }

    pub fn mul(&mut self, n: usize) -> Self {
        Self {
            run_in_parallel: self.run_in_parallel,
            max_concurrent: self.max_concurrent * n,
            requests_per_second: self.requests_per_second * (n as f32),
        }
    }
}