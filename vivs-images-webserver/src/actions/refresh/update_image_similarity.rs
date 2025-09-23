use async_trait::async_trait;
use convert_case::{Case, Casing};
use nameof::name_of_type;
use sqlx::{Pool, Sqlite};
use tokio::sync::Semaphore;
use std::sync::Arc;
use std::time::Duration;

use crate::actions::channels::{task_to_worker_send_helper, TaskToWorkerMessage, TaskToWorkerSender};
use crate::actions::action_registry::IWebServerAction;
use crate::converters::extract_image_similarity::{extract_image_similarity, ComputeImageSimilarityOptions};
use crate::database::query::query_image_similarity::{get_image_paths_from_db, get_image_similarity_value_exists_in_db};
use crate::database::update::update_image_similarity::{execute_delete_image_similarity_sql, execute_insert_image_similarity_sql};
use crate::metrics::similarity_metrics::{get_image_path_comparison_analysis, SimilarityMissingAnalysis};
use crate::models::image_similarity::ImageComparisonAlgorithm;

// Helper struct to manage similarity processing
pub struct SimilarityProcessor;

impl SimilarityProcessor {
    // Process a single image pair for similarity calculation and database update
    pub async fn process_similarity_pair(
        pool: &Pool<Sqlite>,
        send: &TaskToWorkerSender,
        dry_run: bool,
        task_id: u32,
        path_a: &str,
        path_b: &str,
    ) -> actix_web::Result<()> {
        let options = ComputeImageSimilarityOptions {
            algo: ImageComparisonAlgorithm::Magick,
            filter_type: None,
            max_dimension: None,
            image_path_a: path_a.to_string(),
            image_path_b: path_b.to_string(),
        };
        
        let task_result = extract_image_similarity(&options);
        match task_result {
            Ok(similarity) => {
                if dry_run {
                    Self::send_log_info(send, task_id, 
                        format!("Dry run similarity {} for {} x {}", similarity.similarity_value, path_a, path_b))?;
                } else {
                    match execute_insert_image_similarity_sql(&similarity, pool).await {
                        Ok(()) => {
                            Self::send_log_info(send, task_id, 
                                format!("Updated similarity to {} for {} x {}", similarity.similarity_value, path_a, path_b))?;
                        },
                        Err(e) => {
                            Self::send_log_error(send, task_id, 
                                format!("update similarity in db error: {}", e))?;
                        }
                    }
                }
            },
            Err(e) => {
                Self::send_log_error(send, task_id, 
                    format!("extract image similarity error: {}", e))?;
            },
        }
        Ok(())
    }

    // Process all missing similarity pairs with progress tracking
    pub async fn process_all_missing_pairs(
        pool: Pool<Sqlite>,
        send: &TaskToWorkerSender,
        dry_run: bool,
        task_id: u32,
        missing_pairs: Vec<(String, String)>,
        initial_progress: f32,
        progress_range: f32,
    ) -> actix_web::Result<()> {
        let total_pairs = missing_pairs.len();
        
        for (index, (path_a, path_b)) in missing_pairs.into_iter().enumerate() {
            Self::process_similarity_pair(&pool, &send, dry_run, task_id, &path_a, &path_b).await?;
            
            // Update progress
            let progress = if total_pairs > 0 {
                initial_progress + (index as f32 / total_pairs as f32) * progress_range
            } else {
                initial_progress + progress_range
            };
            Self::send_progress_update(&send, task_id, progress)?;
        }
        
        Ok(())
    }

    // Process similarity pairs in parallel with rate limiting
    pub async fn process_pairs_parallel(
        pool: Pool<Sqlite>,
        send: &TaskToWorkerSender,
        dry_run: bool,
        task_id: u32,
        pairs: Vec<(String, String)>,
        initial_progress: f32,
        progress_range: f32,
        max_concurrent: usize,
        requests_per_second: f64,
    ) -> actix_web::Result<()> {
        let semaphore = Arc::new(Semaphore::new(max_concurrent));
        let min_interval = Duration::from_secs_f64(1.0 / requests_per_second);
        let mut interval = tokio::time::interval(min_interval);
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        let total_pairs = pairs.len();
        let mut tasks = Vec::new();

        for (index, (path_a, path_b)) in pairs.into_iter().enumerate() {
            let progress = if total_pairs > 0 {
                initial_progress + (index as f32 / total_pairs as f32) * progress_range
            } else {
                initial_progress + progress_range
            };

            if path_a != path_b && !get_image_similarity_value_exists_in_db(&path_a, &path_b, &pool).await? {
                interval.tick().await; // Rate limiting
                
                let pool_clone = pool.clone();
                let send_clone = send.clone();
                let permit = semaphore.clone().acquire_owned().await
                    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

                let task = tokio::spawn(async move {
                    let _permit = permit;
                    if let Err(e) = Self::process_similarity_pair(
                        &pool_clone, 
                        &send_clone, 
                        dry_run, 
                        task_id, 
                        &path_a, 
                        &path_b
                    ).await {
                        let _ = Self::send_log_error(&send_clone, task_id, format!("{}", e));
                    }

                    // Update progress
                    let _ = Self::send_progress_update(&send_clone, task_id, progress);
                });

                tasks.push(task);
            } else {
                let _ = Self::send_progress_update(&send, task_id, progress);
            }
        }

        // Wait for all tasks to complete
        let results = futures::future::join_all(tasks).await;
        
        // Handle results
        for result in results {
            if let Err(e) = result {
                Self::send_log_error(&send, task_id, format!("Task error: {}", e))?;
            }
        }

        Ok(())
    }

    // Generate all unique pairs from a list of image paths
    pub fn generate_unique_pairs(image_list: &[String]) -> Vec<(String, String)> {
        let mut pairs = Vec::new();
        for (i, path_a) in image_list.iter().enumerate() {
            for path_b in image_list.iter().skip(i + 1) {
                pairs.push((path_a.clone(), path_b.clone()));
            }
        }
        pairs
    }

    // Helper methods for sending messages
    fn send_log_info(send: &TaskToWorkerSender, task_id: u32, message: String) -> actix_web::Result<()> {
        task_to_worker_send_helper(send, TaskToWorkerMessage::LogInfo(task_id, message))
    }

    fn send_log_error(send: &TaskToWorkerSender, task_id: u32, message: String) -> actix_web::Result<()> {
        task_to_worker_send_helper(send, TaskToWorkerMessage::LogError(task_id, message))
    }

    fn send_progress_update(send: &TaskToWorkerSender, task_id: u32, progress: f32) -> actix_web::Result<()> {
        task_to_worker_send_helper(send, TaskToWorkerMessage::ProgressUpdate(task_id, progress))
    }
}

// Base trait for common action functionality
#[async_trait]
pub trait SimilarityActionHelpers {
    fn send_initial_analysis(&self, send: &TaskToWorkerSender, task_id: u32, analysis: &SimilarityMissingAnalysis) -> actix_web::Result<()> {
        task_to_worker_send_helper(send, TaskToWorkerMessage::LogInfo(task_id, analysis.log.clone()))?;
        task_to_worker_send_helper(send, TaskToWorkerMessage::LogError(task_id, analysis.log_error.clone()))?;
        task_to_worker_send_helper(send, TaskToWorkerMessage::LogInfo(task_id, analysis.message.clone()))?;
        Ok(())
    }

    async fn get_missing_pairs_from_analysis(
        &self, 
        pool: &Pool<Sqlite>, 
        analysis: &SimilarityMissingAnalysis
    ) -> actix_web::Result<Vec<(String, String)>> {
        let mut missing_pairs = Vec::new();
        
        for path_a in &analysis.files_missing_from_sql {
            for path_b in &analysis.files_missing_from_sql {
                if path_a != path_b && !get_image_similarity_value_exists_in_db(path_a, path_b, pool).await? {
                    missing_pairs.push((path_a.clone(), path_b.clone()));
                }
            }
        }
        
        Ok(missing_pairs)
    }
}

// Refactored actions using the helper functions
pub struct InsertNewImageSimilarityFromDiskAction {}

impl InsertNewImageSimilarityFromDiskAction {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IWebServerAction for InsertNewImageSimilarityFromDiskAction {
    fn get_name(&self) -> String {
        name_of_type!(InsertNewImageSimilarityFromDiskAction).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(InsertNewImageSimilarityFromDiskAction).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the similarity table is missing any values it will add them.".to_string()
    }

    fn get_is_runnable(&self) -> bool {
        true
    }
    
    fn get_can_dry_run(&self) -> bool { true }
    
    async fn run_task(&self, pool: Pool<Sqlite>, send: TaskToWorkerSender, dry_run: bool, task_id: u32) -> actix_web::Result<()> {
        let analysis = get_image_path_comparison_analysis(&pool).await?;
        SimilarityProcessor::send_log_info(&send, task_id, analysis.log.clone())?;
        SimilarityProcessor::send_log_error(&send, task_id, analysis.log_error.clone())?;
        SimilarityProcessor::send_log_info(&send, task_id, analysis.message.clone())?;

        let missing_pairs = SimilarityProcessor::generate_unique_pairs(&analysis.files_missing_from_sql);
        
        SimilarityProcessor::process_all_missing_pairs(
            pool, &send, dry_run, task_id, missing_pairs, 0.0, 1.0
        ).await?;
        
        SimilarityProcessor::send_progress_update(&send, task_id, 1.0)?;
        Ok(())
    }
}

pub struct InsertNewImageSimilarityFromSqlDbAction {}

impl InsertNewImageSimilarityFromSqlDbAction {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl IWebServerAction for InsertNewImageSimilarityFromSqlDbAction {
    fn get_name(&self) -> String {
        name_of_type!(InsertNewImageSimilarityFromSqlDbAction).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(InsertNewImageSimilarityFromSqlDbAction).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the similarity table has any entries not found for other files in the sql db.".to_string()
    }

    fn get_is_runnable(&self) -> bool {
        true
    }

    fn get_can_dry_run(&self) -> bool { true }
    
    async fn run_task(&self, pool: Pool<Sqlite>, send: TaskToWorkerSender, dry_run: bool, task_id: u32) -> actix_web::Result<()> {
        let image_list = get_image_paths_from_db(&pool).await?;
        SimilarityProcessor::send_progress_update(&send, task_id, 0.0)?;

        let run_in_parallel = true;
        let pairs = SimilarityProcessor::generate_unique_pairs(&image_list);

        if run_in_parallel {
            SimilarityProcessor::process_pairs_parallel(
                pool, &send, dry_run, task_id, pairs, 0.0, 1.0, 8, 16.0
            ).await?;
        } else {
            SimilarityProcessor::process_all_missing_pairs(
                pool, &send, dry_run, task_id, pairs, 0.0, 1.0
            ).await?;
        }

        SimilarityProcessor::send_progress_update(&send, task_id, 1.0)?;
        Ok(())
    }
}

pub struct DeleteImageSimilarityFromSqlNotOnDiskAction {}

impl DeleteImageSimilarityFromSqlNotOnDiskAction {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl IWebServerAction for DeleteImageSimilarityFromSqlNotOnDiskAction {
    fn get_name(&self) -> String {
        name_of_type!(DeleteImageSimilarityFromSqlNotOnDiskAction).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(DeleteImageSimilarityFromSqlNotOnDiskAction).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the similarity table has any values not found on disk it will delete them.".to_string()
    }

    fn get_is_runnable(&self) -> bool {
        true
    }

    fn get_can_dry_run(&self) -> bool { false }
    
    async fn run_task(&self, pool: Pool<Sqlite>, send: TaskToWorkerSender, _dry_run: bool, task_id: u32) -> actix_web::Result<()> {
        let analysis = get_image_path_comparison_analysis(&pool).await?;
        SimilarityProcessor::send_log_info(&send, task_id, analysis.log.clone())?;
        SimilarityProcessor::send_log_error(&send, task_id, analysis.log_error.clone())?;
        SimilarityProcessor::send_log_info(&send, task_id, analysis.message.clone())?;
        SimilarityProcessor::send_progress_update(&send, task_id, 0.3)?;

        let total_files = analysis.files_missing_from_disk.len();
        
        for (index, missing_from_disk) in analysis.files_missing_from_disk.into_iter().enumerate() {
            match execute_delete_image_similarity_sql(&missing_from_disk, &pool).await {
                Ok(()) => {
                    SimilarityProcessor::send_log_info(&send, task_id, 
                        format!("Deleted {} from db", missing_from_disk))?;
                },
                Err(e) => {
                    SimilarityProcessor::send_log_error(&send, task_id, 
                        format!("Delete {} from db error: {}", missing_from_disk, e))?;
                }
            }

            let progress = 0.3 + (index as f32 / total_files as f32) * 0.7;
            SimilarityProcessor::send_progress_update(&send, task_id, progress)?;
        }
        
        SimilarityProcessor::send_progress_update(&send, task_id, 1.0)?;
        Ok(())
    }
}