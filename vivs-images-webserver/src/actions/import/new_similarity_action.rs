use std::collections::HashSet;
use std::io::ErrorKind;
use std::sync::Arc;

use sqlx::{Pool, Sqlite};

use crate::actions::analysis_task_item_processor::LogProgListenerPair;
use crate::calc::file_paths_comparison::CrossFilePathComparisonModel;
use crate::calc::math::calculate_progress;
use crate::converters::extract_image_similarity::extract_image_similarity;
use crate::converters::extract_image_similarity::ComputeImageSimilarityOptions;
use crate::database::query::query_image_similarity::get_image_similarity_value_exists_in_db;
use crate::database::query::query_image_similarity::query_similarity_table_pairs_using_thumbnail_algo;
use crate::database::query::query_image_thumbnail::get_thumbnail_image_paths_from_db;
use crate::database::update::update_image_similarity::execute_insert_image_similarity_sql;
use crate::metrics::similarity_metrics::get_image_paths_full_difference_similarity_analysis;
use crate::models::image_similarity::ImageComparisonAlgorithm;
use crate::models::image_similarity::ImageSimilarity;
use crate::actions::analysis_task_item_processor::AnalysisTaskItemProcessorOrchestrator;
use crate::actions::analysis_task_item_processor::AnalysisTaskItemProcessor;



pub struct SimilarityFromDiskProcessor;
impl SimilarityFromDiskProcessor {
    pub fn new() -> Self { Self {} }
}

use async_trait::async_trait;


#[async_trait]
impl AnalysisTaskItemProcessor<Arc<CrossFilePathComparisonModel>, Arc<ComputeImageSimilarityOptions>, HashSet<Arc<ComputeImageSimilarityOptions>>, Arc<ImageSimilarity>> for SimilarityFromDiskProcessor {
    async fn get_analysis(&self, pool: Pool<Sqlite>, log_prog_listener: Option<LogProgListenerPair>) -> Result<Arc<CrossFilePathComparisonModel>, Box<dyn std::error::Error + Send>> {
        get_image_paths_full_difference_similarity_analysis(&pool, log_prog_listener).await
            .map(|v| Arc::new(v))
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn get_task_items_from_analysis(&self, _pool: Pool<Sqlite>, analysis: Arc<CrossFilePathComparisonModel>, log_prog_listener: Option<LogProgListenerPair>) -> Result<HashSet<Arc<ComputeImageSimilarityOptions>>, Box<dyn std::error::Error + Send>> {
        if let Some(log_prog_listener) = log_prog_listener {
            let total = analysis.pairs_missing_from_b.len();
            Ok(analysis.pairs_missing_from_b
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    if i % 4000 == 0 {
                        log_prog_listener.0(calculate_progress(i, total));
                    }
                    ComputeImageSimilarityOptions::new_defaults(x.0.clone(), x.1.clone())
                })
                .map(Arc::new)
                .collect()
            )
        } else {
            Ok(analysis.pairs_missing_from_b
                .iter()
                .map(|x| ComputeImageSimilarityOptions::new_defaults(x.0.clone(), x.1.clone()))
                .map(Arc::new)
                .collect()
            )
        }
    }

    async fn process_task_item(&self, task_item: Arc<ComputeImageSimilarityOptions>, pool: Pool<Sqlite>) -> Result<Arc<ImageSimilarity>, Box<dyn std::error::Error + Send>> {
        let task_result = extract_image_similarity(&task_item, &pool).await;
        match task_result {
            Ok(similarity) => {
                Ok(Arc::new(similarity))
            },
            Err(e) => {
                Err(Box::new(e) as Box<dyn std::error::Error + Send>)
            },
        }
    }

    async fn process_task_output(&self, task_output: Arc<ImageSimilarity>, pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error + Send>> {
        execute_insert_image_similarity_sql(&task_output, &pool).await
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn task_already_completed(&self, task_input: &Arc<ComputeImageSimilarityOptions>, pool: Pool<Sqlite>) -> Result<bool, Box<dyn std::error::Error + Send>> {
        get_image_similarity_value_exists_in_db(&task_input.image_path_a, &task_input.image_path_b, &pool).await
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    fn get_description(&self) -> String {
        "if the similarity table is missing any entries from the disk, it will add them".to_string()
    }

    fn get_item_name(&self) -> String {
        "similarity".to_string()
    }

    fn get_process_action_name(&self) -> String {
        "add_from_disk".to_string()
    }
}

pub struct InsertNewSimilaritysFromDiskOrchestratorAction;
impl InsertNewSimilaritysFromDiskOrchestratorAction {
    pub fn new() -> AnalysisTaskItemProcessorOrchestrator<Arc<CrossFilePathComparisonModel>, Arc<ComputeImageSimilarityOptions>, HashSet<Arc<ComputeImageSimilarityOptions>>, Arc<ImageSimilarity>> {
        AnalysisTaskItemProcessorOrchestrator::new(Arc::new(SimilarityFromDiskProcessor::new()))
    }
}









pub struct SimilarityFromThumbnailsProcessor;
impl SimilarityFromThumbnailsProcessor {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl AnalysisTaskItemProcessor<Arc<CrossFilePathComparisonModel>, Arc<ComputeImageSimilarityOptions>, HashSet<Arc<ComputeImageSimilarityOptions>>, Arc<ImageSimilarity>> for SimilarityFromThumbnailsProcessor {
    async fn get_analysis(&self, pool: Pool<Sqlite>, log_prog_listener: Option<LogProgListenerPair>) -> Result<Arc<CrossFilePathComparisonModel>, Box<dyn std::error::Error + Send>> {
        if let Some(x) = &log_prog_listener {
            x.1("getting thumbnail image paths from db");
            x.0(0.3);
        }
        let thumbnail_paths = 
        get_thumbnail_image_paths_from_db(&pool).await
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)?;
        
        if let Some(x) = &log_prog_listener {
            x.1("getting similarity image pairs from db");
            x.0(0.6);
        }
        let similarity_pairs_used_thumbnail_algo = query_similarity_table_pairs_using_thumbnail_algo(ImageComparisonAlgorithm::CustomV2Thumbnails, &pool).await
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)?;

        if let Some(x) = &log_prog_listener {
            x.1("getting comparison of image paths");
            x.0(0.9);
        }

        Ok(Arc::new(CrossFilePathComparisonModel::new_easy_2(thumbnail_paths, "thumbnail paths", similarity_pairs_used_thumbnail_algo, "similarity paths", log_prog_listener)))
    }

    async fn get_task_items_from_analysis(&self, _pool: Pool<Sqlite>, analysis: Arc<CrossFilePathComparisonModel>, log_prog_listener: Option<LogProgListenerPair>) -> Result<HashSet<Arc<ComputeImageSimilarityOptions>>, Box<dyn std::error::Error + Send>> {
        if let Some(log_prog_listener) = log_prog_listener {
            let total = analysis.pairs_missing_from_b.len();
            Ok(analysis.pairs_missing_from_b
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    log_prog_listener.0(calculate_progress(i, total));
                    ComputeImageSimilarityOptions {
                        algo: ImageComparisonAlgorithm::CustomV2Thumbnails,
                        max_dimension: Some(32),
                        filter_type: Some(image::imageops::FilterType::Nearest),
                        image_path_a: x.0.clone(),
                        image_path_b: x.1.clone()
                    }
                })
                .map(Arc::new)
                .collect()
            )
        } else {
            Ok(analysis.pairs_missing_from_b
                .iter()
                .map(|x| ComputeImageSimilarityOptions {
                    algo: ImageComparisonAlgorithm::CustomV2Thumbnails,
                    max_dimension: Some(32),
                    filter_type: Some(image::imageops::FilterType::Nearest),
                    image_path_a: x.0.clone(),
                    image_path_b: x.1.clone()
                })
                .map(Arc::new)
                .collect()
            )
        }
    }

    async fn process_task_item(&self, task_item: Arc<ComputeImageSimilarityOptions>, pool: Pool<Sqlite>) -> Result<Arc<ImageSimilarity>, Box<dyn std::error::Error + Send>> {
        let task_result = extract_image_similarity(&task_item, &pool).await;
        match task_result {
            Ok(similarity) => {
                Ok(Arc::new(similarity))
            },
            Err(e) => {
                Err(Box::new(e) as Box<dyn std::error::Error + Send>)
            },
        }
    }

    async fn process_task_output(&self, task_output: Arc<ImageSimilarity>, pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error + Send>> {
        execute_insert_image_similarity_sql(&task_output, &pool).await
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn task_already_completed(&self, task_input: &Arc<ComputeImageSimilarityOptions>, pool: Pool<Sqlite>) -> Result<bool, Box<dyn std::error::Error + Send>> {
        get_image_similarity_value_exists_in_db(&task_input.image_path_a, &task_input.image_path_b, &pool).await
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    fn get_description(&self) -> String {
        "if the similarity table is missing any entries from other tables, it will add them".to_string()
    }

    fn get_item_name(&self) -> String {
        "similarity".to_string()
    }

    fn get_process_action_name(&self) -> String {
        "add_from_db".to_string()
    }
}

pub struct InsertNewSimilaritysFromThumbnailsOrchestratorAction;
impl InsertNewSimilaritysFromThumbnailsOrchestratorAction {
    pub fn new() -> AnalysisTaskItemProcessorOrchestrator<Arc<CrossFilePathComparisonModel>, Arc<ComputeImageSimilarityOptions>, HashSet<Arc<ComputeImageSimilarityOptions>>, Arc<ImageSimilarity>> {
        AnalysisTaskItemProcessorOrchestrator::new(Arc::new(SimilarityFromThumbnailsProcessor::new()))
    }
}