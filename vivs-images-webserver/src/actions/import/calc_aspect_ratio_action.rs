// calc_aspect_ratio_action.rs

use std::collections::HashSet;
use std::io::ErrorKind;
use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

use crate::actions::analysis_task_item_processor::LogProgListenerPair;
use crate::calc::file_paths_comparison::FilePathComparisonModel;
use crate::converters::extract_image_aspect_ratio::extract_image_aspect_ratio_model;
use crate::database::query::query_image_aspect_ratio::query_aspect_ratio_table_count;
use crate::database::update::update_image_aspect_ratio::execute_insert_image_aspect_ratio_sql;
use crate::metrics::aspect_ratio_metrics::get_image_path_comparison_aspect_ratio_table_analysis;
use crate::models::image_aspect_ratio::ImageAspectRatio;
use crate::actions::analysis_task_item_processor::AnalysisTaskItemProcessorOrchestrator;
use crate::actions::analysis_task_item_processor::AnalysisTaskItemProcessor;


pub struct CalcAspectRatioProcessor;
impl CalcAspectRatioProcessor {
    pub fn new() -> Self { Self {} }
}


#[async_trait]
impl AnalysisTaskItemProcessor<Arc<FilePathComparisonModel>, String, HashSet<String>, Arc<ImageAspectRatio>> for CalcAspectRatioProcessor {
    async fn get_analysis(&self, pool: Pool<Sqlite>, log_prog_listener: Option<LogProgListenerPair>) -> Result<Arc<FilePathComparisonModel>, Box<dyn std::error::Error + Send>> {
        get_image_path_comparison_aspect_ratio_table_analysis(&pool, log_prog_listener).await
            .map(|v| Arc::new(v))
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn get_task_items_from_analysis(&self, _pool: Pool<Sqlite>, analysis: Arc<FilePathComparisonModel>, _log_prog_listener: Option<LogProgListenerPair>) -> Result<HashSet<String>, Box<dyn std::error::Error + Send>> {
        Ok(analysis.files_missing_from_b.clone())
    }

    async fn process_task_item(&self, task_item: String, _pool: Pool<Sqlite>) -> Result<Arc<ImageAspectRatio>, Box<dyn std::error::Error + Send>> {
        extract_image_aspect_ratio_model(&task_item)
            .map(Arc::new)
            .map_err(|e| {
                Box::new(e) as Box<dyn std::error::Error + Send>
            })
    }

    async fn process_task_output(&self, task_output: Arc<ImageAspectRatio>, pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error + Send>> {
        execute_insert_image_aspect_ratio_sql(&task_output, &pool).await
            .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)?;
        Ok(())
    }

    async fn task_already_completed(&self, task_input: &String, pool: Pool<Sqlite>) -> Result<bool, Box<dyn std::error::Error + Send>> {
        query_aspect_ratio_table_count(&task_input, &pool).await
            .map(|v| v > 0)
            .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    fn get_description(&self) -> String {
        "if the aspect_ratio table is missing any entries, it will add them".to_string()
    }

    fn get_item_name(&self) -> String {
        "aspect_ratio".to_string()
    }

    fn get_process_action_name(&self) -> String {
        "add".to_string()
    }
}

pub struct InsertNewAspectRatioOrchestratorAction;
impl InsertNewAspectRatioOrchestratorAction {
    pub fn new() -> AnalysisTaskItemProcessorOrchestrator<Arc<FilePathComparisonModel>, String, HashSet<String>, Arc<ImageAspectRatio>> {
        AnalysisTaskItemProcessorOrchestrator::new(Arc::new(CalcAspectRatioProcessor::new()))
    }
}