use std::io::ErrorKind;
use std::sync::Arc;

use sqlx::{Pool, Sqlite};

use crate::calc::file_paths_comparison::FilePathComparisonModel;
use crate::converters::extract_image_similarity::extract_image_similarity;
use crate::converters::extract_image_similarity::ComputeImageSimilarityOptions;
use crate::database::query::query_image_exif::get_image_paths_from_db;
use crate::database::query::query_image_similarity::get_image_similarity_value_exists_in_db;
use crate::database::query::query_image_similarity::get_similarity_table_count_from_db;
use crate::database::query::query_image_similarity::query_similarity_table_paths_using_thumbnail_algo;
use crate::database::query::query_image_thumbnail::get_thumbnail_image_paths_from_db;
use crate::database::update::update_image_similarity::execute_insert_image_similarity_sql;
use crate::metrics::similarity_metrics::get_image_path_comparison_analysis;
use crate::models::image_similarity::ImageComparisonAlgorithm;
use crate::models::image_similarity::ImageSimilarity;
use crate::actions::refresh::analysis_task_item_processor::AnalysisTaskItemProcessorOrchestrator;
use crate::actions::refresh::analysis_task_item_processor::AnalysisTaskItemProcessor;



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


pub struct SimilarityFromDiskProcessor;
impl SimilarityFromDiskProcessor {
    pub fn new() -> Self { Self {} }
}

use async_trait::async_trait;


#[async_trait]
impl AnalysisTaskItemProcessor<Arc<FilePathComparisonModel>, Arc<ComputeImageSimilarityOptions>, Arc<ImageSimilarity>> for SimilarityFromDiskProcessor {
    async fn get_analysis(&self, pool: Pool<Sqlite>) -> Result<Arc<FilePathComparisonModel>, Box<dyn std::error::Error + Send>> {
        get_image_path_comparison_analysis(&pool).await
            .map(|v| Arc::new(v))
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn get_task_items_from_analysis(&self, _pool: Pool<Sqlite>, analysis: Arc<FilePathComparisonModel>) -> Result<Vec<Arc<ComputeImageSimilarityOptions>>, Box<dyn std::error::Error + Send>> {
        Ok(generate_unique_pairs(&analysis.files_missing_from_a)
            .into_iter()
            .map(|x| ComputeImageSimilarityOptions::new_defaults(x.0, x.1))
            .map(Arc::new)
            .collect()
        )
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

    async fn exists_in_db(&self, task_input: &Arc<ComputeImageSimilarityOptions>, pool: Pool<Sqlite>) -> Result<bool, Box<dyn std::error::Error + Send>> {
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
    pub fn new2() -> AnalysisTaskItemProcessorOrchestrator<Arc<FilePathComparisonModel>, Arc<ComputeImageSimilarityOptions>, Arc<ImageSimilarity>> {
        AnalysisTaskItemProcessorOrchestrator::new(Arc::new(SimilarityFromDiskProcessor::new()))
    }
}









pub struct SimilarityFromThumbnailsProcessor;
impl SimilarityFromThumbnailsProcessor {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl AnalysisTaskItemProcessor<Arc<FilePathComparisonModel>, Arc<ComputeImageSimilarityOptions>, Arc<ImageSimilarity>> for SimilarityFromThumbnailsProcessor {
    async fn get_analysis(&self, pool: Pool<Sqlite>) -> Result<Arc<FilePathComparisonModel>, Box<dyn std::error::Error + Send>> {
        let thumbnail_paths = 
        get_thumbnail_image_paths_from_db(&pool).await
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)?;
        let similarity_paths_used_thumbnail_algo = query_similarity_table_paths_using_thumbnail_algo(ImageComparisonAlgorithm::CustomV2Thumbnails, &pool).await
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)?;
        Ok(Arc::new(FilePathComparisonModel::new(thumbnail_paths, "thumbnail paths", similarity_paths_used_thumbnail_algo, "similarity paths")))
    }

    async fn get_task_items_from_analysis(&self, _pool: Pool<Sqlite>, analysis: Arc<FilePathComparisonModel>) -> Result<Vec<Arc<ComputeImageSimilarityOptions>>, Box<dyn std::error::Error + Send>> {
        Ok(generate_unique_pairs(&analysis.files_missing_from_a)
            .into_iter()
            .map(|x| ComputeImageSimilarityOptions {
                algo: ImageComparisonAlgorithm::CustomV2Thumbnails,
                max_dimension: Some(32),
                filter_type: Some(image::imageops::FilterType::Nearest),
                image_path_a: x.0,
                image_path_b: x.1
            })
            .map(Arc::new)
            .collect()
        )
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

    async fn exists_in_db(&self, task_input: &Arc<ComputeImageSimilarityOptions>, pool: Pool<Sqlite>) -> Result<bool, Box<dyn std::error::Error + Send>> {
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

pub type InsertNewSimilaritysFromThumbnailsOrchestratorAction = AnalysisTaskItemProcessorOrchestrator<Arc<FilePathComparisonModel>, Arc<ComputeImageSimilarityOptions>, Arc<ImageSimilarity>>;
impl InsertNewSimilaritysFromThumbnailsOrchestratorAction {
    pub fn new2() -> Self {
        AnalysisTaskItemProcessorOrchestrator::new(Arc::new(SimilarityFromThumbnailsProcessor::new()))
    }
}