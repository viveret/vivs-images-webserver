use async_trait::async_trait;
use convert_case::{Case, Casing};
use nameof::name_of_type;
use sqlx::{Pool, Sqlite};

use tokio::sync::Semaphore;
use tokio::task;
use std::clone;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use crate::actions::channels::{task_to_worker_send_helper, TaskToWorkerMessage, TaskToWorkerSender};
use crate::actions::action_registry::IWebServerAction;
use crate::converters::extract_image_similarity::{extract_image_similarity, ComputeImageSimilarityOptions};
use crate::database::query::query_image_similarity::{get_image_paths_from_db, get_image_similarity_value_exists_in_db};
use crate::database::update::update_image_similarity::{execute_delete_image_similarity_sql, execute_insert_image_similarity_sql};
use crate::metrics::similarity_metrics::{get_image_path_comparison_analysis, SimilarityMissingAnalysis};
use crate::models::image_similarity::ImageComparisonAlgorithm;

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
        "If the Similarity table is missing any values it will add them.".to_string()
    }

    fn get_is_runnable(&self) -> bool {
        true
    }
    
    fn get_can_dry_run(&self) -> bool { true }
    
    async fn run_task(&self, pool: Pool<Sqlite>, send: TaskToWorkerSender, dry_run: bool, task_id: u32) -> actix_web::Result<()> {
        // first part is getting the file list
        let analysis = get_image_path_comparison_analysis(&pool).await?;
        let analysis: SimilarityMissingAnalysis = (*analysis).clone();
        task_to_worker_send_helper(&send, TaskToWorkerMessage::LogInfo(task_id, analysis.log))?;
        task_to_worker_send_helper(&send, TaskToWorkerMessage::LogError(task_id, analysis.log_error))?;
        task_to_worker_send_helper(&send, TaskToWorkerMessage::LogInfo(task_id, analysis.message))?;

        // most of the work is calculating the values and updating the db
        let mut missing_index = 0;
        let missing_count = analysis.files_missing_from_sql.len();
        let missing_count = missing_count * (missing_count - 1) / 2;
        for missing_from_db_a in analysis.files_missing_from_sql.iter() {
            for missing_from_db_b in analysis.files_missing_from_sql.iter() {
                let exists_in_db = missing_from_db_a == missing_from_db_b || get_image_similarity_value_exists_in_db(missing_from_db_a, missing_from_db_b, &pool).await?;
                if !exists_in_db {
                    let options = ComputeImageSimilarityOptions {
                        algo: ImageComparisonAlgorithm::Magick,
                        filter_type: None,
                        max_dimension: None,
                        image_path_a: missing_from_db_a.clone(),
                        image_path_b: missing_from_db_b.clone(),
                    };                                                                                                                                                                                                                                          
                    let task_result = extract_image_similarity(&options);
                    match task_result {
                        Ok(similarity) => {
                            if dry_run {
                                task_to_worker_send_helper(&send, TaskToWorkerMessage::LogInfo(task_id, format!("Dry run similarity {} for {} x {}", similarity.similarity_value, missing_from_db_a, missing_from_db_b)))?;
                            } else {
                                match execute_insert_image_similarity_sql(&similarity, &pool).await {
                                    Ok(()) => {
                                        task_to_worker_send_helper(&send, TaskToWorkerMessage::LogInfo(task_id, format!("Updated similarity to {} for {} x {}", similarity.similarity_value, missing_from_db_a, missing_from_db_b)))?;
                                    },
                                    Err(e) => {
                                        task_to_worker_send_helper(&send, TaskToWorkerMessage::LogError(task_id, format!("update db image Similarity error: {}", e)))?;
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            task_to_worker_send_helper(&send, TaskToWorkerMessage::LogError(task_id, format!("extract image Similarity error: {}", e)))?;
                        },
                    }
                }

                missing_index = missing_index + 1;
                let missing_progress = (missing_index as f64) / (missing_count as f64);
                let new_progress = missing_progress;
                task_to_worker_send_helper(&send, TaskToWorkerMessage::ProgressUpdate(task_id, new_progress as f32))?;
            }
        }
        
        task_to_worker_send_helper(&send, TaskToWorkerMessage::ProgressUpdate(task_id, 1.0))?;
        Ok(())
    }
}





pub struct InsertNewImageSimilarityFromSqlDbAction {}
impl InsertNewImageSimilarityFromSqlDbAction {
    pub fn new() -> Self { Self {} }
    
    async fn run_task_linear(&self, pool: Pool<Sqlite>, send: TaskToWorkerSender, dry_run: bool, task_id: u32) -> actix_web::Result<()> {
        // first part is getting the file list
        let image_list = get_image_paths_from_db(&pool).await?;
        task_to_worker_send_helper(&send, TaskToWorkerMessage::ProgressUpdate(task_id, 0.0))?;

        // most of the work is calculating the values and updating the db
        let mut missing_index = 0;
        let image_list_len = image_list.len();
        let missing_count = image_list_len * (image_list_len - 1) / 2;
        for missing_from_db_a in image_list.iter() {
            for missing_from_db_b in image_list.iter() {
                let exists_in_db = missing_from_db_a == missing_from_db_b || get_image_similarity_value_exists_in_db(missing_from_db_a, missing_from_db_b, &pool).await?;
                if !exists_in_db {
                    let options = ComputeImageSimilarityOptions {
                        algo: ImageComparisonAlgorithm::Magick,
                        filter_type: None,
                        max_dimension: None,
                        image_path_a: missing_from_db_a.clone(),
                        image_path_b: missing_from_db_b.clone(),
                    };
                    let task_result = extract_image_similarity(&options);
                    match task_result {
                        Ok(similarity) => {
                            if dry_run {
                                task_to_worker_send_helper(&send, TaskToWorkerMessage::LogInfo(task_id, format!("Dry run similarity {} for {} x {}", similarity.similarity_value, missing_from_db_a, missing_from_db_b)))?;
                            } else {
                                match execute_insert_image_similarity_sql(&similarity, &pool).await {
                                    Ok(()) => {
                                        task_to_worker_send_helper(&send, TaskToWorkerMessage::LogInfo(task_id, format!("Updated similarity to {} for {} x {}", similarity.similarity_value, missing_from_db_a, missing_from_db_b)))?;
                                    },
                                    Err(e) => {
                                        task_to_worker_send_helper(&send, TaskToWorkerMessage::LogError(task_id, format!("update db image Similarity error: {}", e)))?;
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            task_to_worker_send_helper(&send, TaskToWorkerMessage::LogError(task_id, format!("extract image Similarity error: {}", e)))?;
                        },
                    }
                }

                missing_index = missing_index + 1;
                let missing_progress = (missing_index as f64) / (missing_count as f64);
                let new_progress = missing_progress;
                task_to_worker_send_helper(&send, TaskToWorkerMessage::ProgressUpdate(task_id, new_progress as f32))?;
            }
        }
        
        task_to_worker_send_helper(&send, TaskToWorkerMessage::ProgressUpdate(task_id, 1.0))?;
        Ok(())
    }

    async fn run_task_parallelized(&self, pool: Pool<Sqlite>, send: TaskToWorkerSender, dry_run: bool, task_id: u32) -> actix_web::Result<()> {
        // First part remains the same
        let image_list = get_image_paths_from_db(&pool).await?;

        // Collect all work items
        let mut tasks = Vec::new();
        let max_concurrent = 8; // Maximum parallel tasks
        let requests_per_second = 16.0; // Rate limit
        let min_interval = Duration::from_secs_f64(1.0 / requests_per_second);

        let semaphore = Arc::new(Semaphore::new(max_concurrent));
        let mut interval = tokio::time::interval(min_interval);
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        // track progress
        let mut missing_index = 0;
        let missing_count = image_list.len();
        let total_pairs = missing_count * (missing_count - 1) / 2;

        for (i, missing_from_db_a) in image_list.iter().enumerate() {
            for (j, missing_from_db_b) in image_list.iter().enumerate() {
                if i >= j { continue; } // Avoid duplicates and self-comparisons

                let pool_clone = pool.clone();
                let send_clone = send.clone();
                let path_a = missing_from_db_a.clone();
                let path_b = missing_from_db_b.clone();
                let current_index = missing_index;
                let task_id = task_id;

                missing_index += 1;

                let exists_in_db = get_image_similarity_value_exists_in_db(&path_a, &path_b, &pool).await?;
                
                if !exists_in_db {
                    interval.tick().await; // Rate limiting
                    let permit = semaphore.clone().acquire_owned().await
                        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
                    let task = tokio::spawn(async move {
                        let _permit = permit;
                        if let Err(e) = Self::process_similarity_pair(
                            pool_clone, 
                            &send_clone, 
                            dry_run, 
                            task_id, 
                            &path_a, 
                            &path_b, 
                            current_index, 
                            total_pairs
                        ).await {
                            let _ = task_to_worker_send_helper(&send_clone, TaskToWorkerMessage::Error(task_id, format!("{}", e)));
                        }
                    });

                    tasks.push(task);
                } else {
                    // Update progress
                    let progress = (current_index as f64) / (total_pairs as f64);
                    task_to_worker_send_helper(&send, TaskToWorkerMessage::ProgressUpdate(task_id, progress as f32))?;
                }
            }
        }

        // Wait for all tasks to complete
        let results = futures::future::join_all(tasks).await;
        
        // Handle results
        for result in results {
            if let Err(e) = result {
                task_to_worker_send_helper(&send, TaskToWorkerMessage::LogError(task_id, format!("Task error: {}", e)))?;
            }
        }

        task_to_worker_send_helper(&send, TaskToWorkerMessage::ProgressUpdate(task_id, 1.0))?;
        Ok(())
    }

    async fn process_similarity_pair(
        pool: Pool<Sqlite>,
        send: &TaskToWorkerSender,
        dry_run: bool,
        task_id: u32,
        path_a: &str,
        path_b: &str,
        current_index: usize,
        total_pairs: usize,
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
                    task_to_worker_send_helper(&send, TaskToWorkerMessage::LogInfo(task_id, 
                        format!("Dry run similarity {} for {} x {}", similarity.similarity_value, path_a, path_b)))?;
                } else {
                    match execute_insert_image_similarity_sql(&similarity, &pool).await {
                        Ok(()) => {
                            task_to_worker_send_helper(&send, TaskToWorkerMessage::LogInfo(task_id, 
                                format!("Updated similarity to {} for {} x {}", similarity.similarity_value, path_a, path_b)))?;
                        },
                        Err(e) => {
                            task_to_worker_send_helper(&send, TaskToWorkerMessage::LogError(task_id, 
                                format!("update db image Similarity error: {}", e)))?;
                        }
                    }
                }
            },
            Err(e) => {
                task_to_worker_send_helper(&send, TaskToWorkerMessage::LogError(task_id, 
                    format!("extract image Similarity error: {}", e)))?;
            },
        }

        // Update progress
        let progress = (current_index as f64) / (total_pairs as f64);
        task_to_worker_send_helper(&send, TaskToWorkerMessage::ProgressUpdate(task_id, progress as f32))?;
        Ok(())
    }
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
        "If the Similarity table has any entries not found for other files in the sql db.".to_string()
    }

    fn get_is_runnable(&self) -> bool {
        true
    }

    fn get_can_dry_run(&self) -> bool { true }
    
    async fn run_task(&self, pool: Pool<Sqlite>, send: TaskToWorkerSender, dry_run: bool, task_id: u32) -> actix_web::Result<()> {
        let run_in_parallel = true;
        if !run_in_parallel {
            self.run_task_linear(pool, send, dry_run, task_id).await
        } else {
            self.run_task_parallelized(pool, send, dry_run, task_id).await
        }
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
        "If the Similarity table has any values not found on disk it will delete them.".to_string()
    }

    fn get_is_runnable(&self) -> bool {
        true
    }

    fn get_can_dry_run(&self) -> bool { false }
    
    async fn run_task(&self, pool: Pool<Sqlite>, send: TaskToWorkerSender, dry_run: bool, task_id: u32) -> actix_web::Result<()> {
        // first 1/3 of progress is getting the difference list
        let analysis = get_image_path_comparison_analysis(&pool).await?;
        let analysis: SimilarityMissingAnalysis = (*analysis).clone();
        task_to_worker_send_helper(&send, TaskToWorkerMessage::LogInfo(task_id, analysis.log))?;
        task_to_worker_send_helper(&send, TaskToWorkerMessage::LogError(task_id, analysis.log_error))?;
        task_to_worker_send_helper(&send, TaskToWorkerMessage::LogInfo(task_id, analysis.message))?;
        task_to_worker_send_helper(&send, TaskToWorkerMessage::ProgressUpdate(task_id, 0.3))?;

        // last 2/3 of progress are deleting rows
        let mut missing_index = 0;
        let missing_count = analysis.files_missing_from_disk.len();
        for missing_from_disk in analysis.files_missing_from_disk {
            match execute_delete_image_similarity_sql(&missing_from_disk, &pool).await {
                Ok(()) => {
                    task_to_worker_send_helper(&send, TaskToWorkerMessage::LogInfo(task_id, format!("Deleted {} from db", missing_from_disk)))?;
                },
                Err(e) => {
                    task_to_worker_send_helper(&send, TaskToWorkerMessage::LogError(task_id, format!("Delete {} from db error: {}", missing_from_disk, e)))?;
                }
            }

            missing_index = missing_index + 1;
            let missing_progress = (missing_index as f64) / (missing_count as f64);
            let new_progress = 0.3 + missing_progress * 0.7;
            task_to_worker_send_helper(&send, TaskToWorkerMessage::ProgressUpdate(task_id, new_progress as f32))?;
        }
        
        task_to_worker_send_helper(&send, TaskToWorkerMessage::ProgressUpdate(task_id, 1.0))?;
        Ok(())
    }
}