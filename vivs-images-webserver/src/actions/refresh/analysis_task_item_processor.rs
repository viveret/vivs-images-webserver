use std::io::ErrorKind;
use std::sync::Arc;
use std::time::Duration;

use convert_case::Case;
use convert_case::Casing;
use nameof::name_of_type;
use sqlx::{Pool, Sqlite};
use tokio::sync::Semaphore;
use async_trait::async_trait;

use crate::actions::channels::TaskToWorkerSender;
use crate::actions::channels::TaskToWorkerMessage;
use crate::actions::channels::task_to_worker_send_helper;
use crate::actions::action_registry::IWebServerAction;

#[async_trait]
pub trait AnalysisTaskItemProcessor<TAnalysis, TTaskItem, TTaskOutput>: Send + Sync 
where 
    TTaskItem: Send + Sync,
    TTaskOutput: Send + Sync,
    TAnalysis: Send + Sync,
{
    async fn get_analysis(&self, pool: Pool<Sqlite>) -> actix_web::Result<TAnalysis, Box<dyn std::error::Error + Send>>;
    async fn get_task_items_from_analysis(&self, pool: Pool<Sqlite>, analysis: TAnalysis) -> actix_web::Result<Vec<TTaskItem>, Box<dyn std::error::Error + Send>>;
    async fn process_task_item(&self, task_item: TTaskItem) -> actix_web::Result<TTaskOutput, Box<dyn std::error::Error + Send>>;
    async fn process_task_output(&self, task_output: TTaskOutput, pool: Pool<Sqlite>) -> actix_web::Result<(), Box<dyn std::error::Error + Send>>;
    async fn exists_in_db(&self, task_input: &TTaskItem, pool: Pool<Sqlite>) -> actix_web::Result<bool, Box<dyn std::error::Error + Send>>;
    fn get_description(&self) -> String;
    fn get_item_name(&self) -> String;
    fn get_process_action_name(&self) -> String;
}

pub struct AnalysisTaskItemProcessorOrchestrator<TAnalysis, TTaskItem, TTaskOutput> 
where
    TTaskItem: Send + Sync,
    TTaskOutput: Send + Sync,
    TAnalysis: Send + Sync,
{
    processor: Arc<dyn AnalysisTaskItemProcessor<TAnalysis, TTaskItem, TTaskOutput>>,
}

impl<TAnalysis, TTaskItem, TTaskOutput> AnalysisTaskItemProcessorOrchestrator<TAnalysis, TTaskItem, TTaskOutput> 
where
    TTaskItem: Send + Sync + std::fmt::Debug + 'static,
    TTaskOutput: Send + Sync + 'static,
    TAnalysis: Send + Sync + 'static,
{
    pub fn new(processor: Arc<dyn AnalysisTaskItemProcessor<TAnalysis, TTaskItem, TTaskOutput>>) -> Self {
        Self {
            processor
        }
    }

    pub async fn process_task_item(
        processor: Arc<dyn AnalysisTaskItemProcessor<TAnalysis, TTaskItem, TTaskOutput>>,
        pool: Pool<Sqlite>,
        send: TaskToWorkerSender,
        dry_run: bool,
        task_id: u32,
        task_input: TTaskItem,
    ) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        let task_item_debug_str = format!("{:?}", task_input);
        let task_result = processor.process_task_item(task_input).await;
        match task_result {
            Ok(task_output) => {
                if dry_run {
                    Self::send_log_info(&send, task_id, 
                        format!("Dry run for {}", task_item_debug_str))?;
                } else {
                    match processor.process_task_output(task_output, pool.clone()).await {
                        Ok(()) => {
                            Self::send_log_info(&send, task_id, 
                                format!("{} processed successfully", processor.get_item_name()))?;
                        },
                        Err(e) => {
                            Self::send_log_error(&send, task_id, 
                                format!("process {} output error: {}", processor.get_item_name(), e))?;
                        }
                    }
                }
            },
            Err(e) => {
                Self::send_log_error(&send, task_id, 
                    format!("process {} error: {}", processor.get_item_name(), e))?;
            },
        }
        Ok(())
    }

    // Process all missing similarity tasks with progress tracking
    pub async fn process_tasks_linear(
        &self,
        pool: Pool<Sqlite>,
        send: &TaskToWorkerSender,
        dry_run: bool,
        task_id: u32,
        missing_tasks: Vec<TTaskItem>,
    ) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        let total_tasks = missing_tasks.len();
        
        for (index, task_item) in missing_tasks.into_iter().enumerate() {
            Self::process_task_item(
                self.processor.clone(),
                pool.clone(), 
                send.clone(), 
                dry_run, 
                task_id, 
                task_item
            ).await?;
            
            // Update progress
            let progress = if total_tasks > 0 {
                (index + 1) as f32 / total_tasks as f32
            } else {
                0.0
            };
            Self::send_progress_update(send, task_id, progress)?;
        }
        
        Ok(())
    }

    // Process similarity tasks in parallel with rate limiting
    pub async fn process_tasks_parallel(
        &self,
        pool: Pool<Sqlite>,
        send: &TaskToWorkerSender,
        dry_run: bool,
        task_id: u32,
        tasks: Vec<TTaskItem>,
        max_concurrent: usize,
        requests_per_second: f64,
    ) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        let semaphore = Arc::new(Semaphore::new(max_concurrent));
        let min_interval = Duration::from_secs_f64(1.0 / requests_per_second);
        let mut interval = tokio::time::interval(min_interval);
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        let total_tasks = tasks.len();
        let mut join_handles = Vec::new();

        for (index, task_input) in tasks.into_iter().enumerate() {
            let progress = if total_tasks > 0 {
                (index + 1) as f32 / total_tasks as f32
            } else {
                0.0
            };

            if !self.processor.exists_in_db(&task_input, pool.clone()).await? {
                interval.tick().await; // Rate limiting
                
                let pool_clone = pool.clone();
                let send_clone = send.clone();
                let processor_clone = self.processor.clone();
                let permit = semaphore.clone().acquire_owned().await
                    .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)?;

                let join_handle = tokio::spawn(async move {
                    let _permit = permit;
                    if let Err(e) = Self::process_task_item(
                        processor_clone,
                        pool_clone, 
                        send_clone.clone(), 
                        dry_run, 
                        task_id, 
                        task_input
                    ).await {
                        let _ = Self::send_log_error(&send_clone, task_id, format!("{}", e));
                    }

                    // Update progress
                    let _ = Self::send_progress_update(&send_clone, task_id, progress);
                });

                join_handles.push(join_handle);
            } else {
                Self::send_progress_update(send, task_id, progress)?;
            }
        }

        // Wait for all tasks to complete
        let results = futures::future::join_all(join_handles).await;
        
        // Handle results
        for result in results {
            if let Err(e) = result {
                Self::send_log_error(send, task_id, format!("Task error: {}", e))?;
            }
        }

        Ok(())
    }

    // Helper methods for sending messages
    fn send_log_info(send: &TaskToWorkerSender, task_id: u32, message: String) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        task_to_worker_send_helper(send, TaskToWorkerMessage::LogInfo(task_id, message))
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    fn send_log_error(send: &TaskToWorkerSender, task_id: u32, message: String) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        task_to_worker_send_helper(send, TaskToWorkerMessage::LogError(task_id, message))
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    fn send_progress_update(send: &TaskToWorkerSender, task_id: u32, progress: f32) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        task_to_worker_send_helper(send, TaskToWorkerMessage::ProgressUpdate(task_id, progress))
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn get_task_items_from_analysis(
        &self, 
        pool: Pool<Sqlite>, 
        analysis: TAnalysis
    ) -> actix_web::Result<Vec<TTaskItem>, Box<dyn std::error::Error + Send>> {
        self.processor.get_task_items_from_analysis(pool, analysis).await
    }

    async fn run_task_parallel_option(&self, pool: Pool<Sqlite>, send: TaskToWorkerSender, dry_run: bool, task_id: u32) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        let analysis = self.processor.get_analysis(pool.clone()).await?;
        
        let task_items = self.get_task_items_from_analysis(pool.clone(), analysis).await?;
        Self::send_progress_update(&send, task_id, 0.0)?;

        let run_in_parallel = true;
        
        if run_in_parallel {
            self.process_tasks_parallel(
                pool, &send, dry_run, task_id, task_items, 8, 16.0
            ).await?;
        } else {
            self.process_tasks_linear(
                pool, &send, dry_run, task_id, task_items
            ).await?;
        }
        
        Self::send_progress_update(&send, task_id, 1.0)?;
        Ok(())
    }
}

#[async_trait]
impl<TAnalysis, TTaskItem, TTaskOutput> IWebServerAction for AnalysisTaskItemProcessorOrchestrator<TAnalysis, TTaskItem, TTaskOutput> 
where
    TTaskItem: Send + Sync + std::fmt::Debug + 'static,
    TTaskOutput: Send + Sync + 'static,
    TAnalysis: Send + Sync + 'static,
{
    fn get_name(&self) -> String {
        format!("{}_{}_{}",
            name_of_type!(AnalysisTaskItemProcessorOrchestrator<TAnalysis, TTaskItem, TTaskOutput>).to_case(Case::Snake),
            self.processor.get_process_action_name().to_case(Case::Snake),
            self.processor.get_item_name().to_case(Case::Snake)
        )
    }

    fn get_label(&self) -> String {
        format!("{} {} {}",
            name_of_type!(AnalysisTaskItemProcessorOrchestrator<TAnalysis, TTaskItem, TTaskOutput>).to_case(Case::Sentence),
            self.processor.get_process_action_name().to_case(Case::Sentence),
            self.processor.get_item_name().to_case(Case::Sentence)
        )
    }

    fn get_description(&self) -> String {
        self.processor.get_description()
    }

    fn get_is_runnable(&self) -> bool { true }
    
    fn get_can_dry_run(&self) -> bool { true }
    
    async fn run_task(&self, pool: Pool<Sqlite>, send: TaskToWorkerSender, dry_run: bool, task_id: u32) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        self.run_task_parallel_option(pool, send, dry_run, task_id).await
    }
}