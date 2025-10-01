// new_exif_action.rs

use std::collections::HashSet;
use std::io::ErrorKind;
use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

use crate::actions::analysis_task_item_processor::LogProgListenerPair;
use crate::calc::file_paths_comparison::FilePathComparisonModel;
use crate::converters::extract_image_exif::extract_image_exif;
use crate::converters::extract_image_exif::ImageToExifAlgo;
use crate::converters::extract_image_exif::ImageToExifOptions;
use crate::database::query::query_image_exif::query_exif_table_count;
use crate::database::update::update_image_exif::execute_insert_image_exif_sql;
use crate::metrics::exif_metrics::get_image_path_comparison_exif_table_analysis;
use crate::models::image_exif::ImageExif;
use crate::actions::analysis_task_item_processor::AnalysisTaskItemProcessorOrchestrator;
use crate::actions::analysis_task_item_processor::AnalysisTaskItemProcessor;



pub struct ExifProcessor;
impl ExifProcessor {
    pub fn new() -> Self { Self {} }
}


#[async_trait]
impl AnalysisTaskItemProcessor<Arc<FilePathComparisonModel>, String, HashSet<String>, Arc<ImageExif>> for ExifProcessor {
    async fn get_analysis(&self, pool: Pool<Sqlite>, log_prog_listener: Option<LogProgListenerPair>) -> Result<Arc<FilePathComparisonModel>, Box<dyn std::error::Error + Send>> {
        get_image_path_comparison_exif_table_analysis(&pool).await
            .map(|v| Arc::new(v))
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn get_task_items_from_analysis(&self, _pool: Pool<Sqlite>, analysis: Arc<FilePathComparisonModel>, log_prog_listener: Option<LogProgListenerPair>) -> Result<HashSet<String>, Box<dyn std::error::Error + Send>> {
        Ok(analysis.files_missing_from_a.clone())
    }

    async fn process_task_item(&self, task_item: String, _pool: Pool<Sqlite>) -> Result<Arc<ImageExif>, Box<dyn std::error::Error + Send>> {
        let options = ImageToExifOptions {
            algo: ImageToExifAlgo::SimpleExifRS
        };
        
        extract_image_exif(&task_item, &options)
            .map(Arc::new)
            .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn process_task_output(&self, task_output: Arc<ImageExif>, pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error + Send>> {
        execute_insert_image_exif_sql((*task_output).clone(), pool).await
            .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn task_already_completed(&self, task_input: &String, pool: Pool<Sqlite>) -> Result<bool, Box<dyn std::error::Error + Send>> {
        query_exif_table_count(&task_input, &pool).await
            .map(|v| v > 0)
            .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    fn get_description(&self) -> String {
        "if the exif table is missing any entries, it will add them".to_string()
    }

    fn get_item_name(&self) -> String {
        "exif".to_string()
    }

    fn get_process_action_name(&self) -> String {
        "add".to_string()
    }
}

pub type InsertNewExifsOrchestratorAction = AnalysisTaskItemProcessorOrchestrator<Arc<FilePathComparisonModel>, String, HashSet<String>, Arc<ImageExif>>;
impl InsertNewExifsOrchestratorAction {
    pub fn new2() -> Self {
        AnalysisTaskItemProcessorOrchestrator::new(Arc::new(ExifProcessor::new()))
    }
}