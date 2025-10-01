// delete_missing_brightness_action.rs

use std::collections::HashSet;
use std::io::ErrorKind;
use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

use crate::actions::refresh::analysis_task_item_processor::AnalysisTaskItemProcessor;
use crate::actions::refresh::analysis_task_item_processor::AnalysisTaskItemProcessorOrchestrator;
use crate::actions::refresh::analysis_task_item_processor::LogProgListenerPair;
use crate::calc::file_paths_comparison::FilePathComparisonModel;
use crate::database::query::query_image_brightness::query_brightness_table_count;
use crate::database::update::update_image_brightness::execute_delete_image_brightness_sql;
use crate::metrics::brightness_metrics::get_image_path_comparison_brightness_table_analysis;



pub struct BrightnessProcessor;
impl BrightnessProcessor {
    pub fn new() -> Self { Self {} }
}


#[async_trait]
impl AnalysisTaskItemProcessor<Arc<FilePathComparisonModel>, String, HashSet<String>, String> for BrightnessProcessor {
    async fn get_analysis(&self, pool: Pool<Sqlite>, log_prog_listener: Option<LogProgListenerPair>) -> Result<Arc<FilePathComparisonModel>, Box<dyn std::error::Error + Send>> {
        get_image_path_comparison_brightness_table_analysis(&pool, log_prog_listener).await
            .map(|v| Arc::new(v))
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn get_task_items_from_analysis(&self, _pool: Pool<Sqlite>, analysis: Arc<FilePathComparisonModel>, _log_prog_listener: Option<LogProgListenerPair>) -> Result<HashSet<String>, Box<dyn std::error::Error + Send>> {
        Ok(analysis.files_missing_from_b.clone())
    }

    async fn process_task_item(&self, task_item: String, _pool: Pool<Sqlite>) -> Result<String, Box<dyn std::error::Error + Send>> {
        Ok(task_item)
    }

    async fn process_task_output(&self, task_output: String, pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error + Send>> {
        execute_delete_image_brightness_sql(&task_output, &pool).await
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn exists_in_db(&self, task_input: &String, pool: Pool<Sqlite>) -> Result<bool, Box<dyn std::error::Error + Send>> {
        query_brightness_table_count(&task_input, &pool).await
            .map(|v| v > 0)
            .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    fn get_description(&self) -> String {
        "if the brightness table has any entries missing from disk, it will delete them".to_string()
    }

    fn get_item_name(&self) -> String {
        "brightness".to_string()
    }

    fn get_process_action_name(&self) -> String {
        "delete_missing".to_string()
    }
}

pub struct DeleteMissingBrightnessOrchestratorAction;
impl DeleteMissingBrightnessOrchestratorAction {
    pub fn new2() -> AnalysisTaskItemProcessorOrchestrator<Arc<FilePathComparisonModel>, String, HashSet<String>, String> {
        AnalysisTaskItemProcessorOrchestrator::new(Arc::new(BrightnessProcessor::new()))
    }
}