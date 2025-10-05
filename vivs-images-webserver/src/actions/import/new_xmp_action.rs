// new_xmp_action.rs

use std::collections::HashSet;
use std::io::ErrorKind;
use std::sync::Arc;

use async_trait::async_trait;

use crate::core::data_context::WebServerActionDataContext;
use crate::actions::analysis_task_item_processor::LogProgListenerPair;
use crate::calc::file_paths_comparison::FilePathComparisonModel;
use crate::converters::extract_image_xmp::extract_image_xmp_model;
use crate::database::query::query_image_xmp::query_xmp_table_count;
use crate::database::update::update_image_xmp::execute_insert_image_xmp_sql;
use crate::metrics::xmp_metrics::get_image_path_comparison_xmp_table_analysis;
use crate::models::image_xmp::ImageXmp;
use crate::actions::analysis_task_item_processor::AnalysisTaskItemProcessorOrchestrator;
use crate::actions::analysis_task_item_processor::AnalysisTaskItemProcessor;


pub struct XmpProcessor;
impl XmpProcessor {
    pub fn new() -> Self { Self {} }
}


#[async_trait]
impl AnalysisTaskItemProcessor<Arc<FilePathComparisonModel>, String, HashSet<String>, Arc<ImageXmp>> for XmpProcessor {
    async fn get_analysis(&self, pool: WebServerActionDataContext, log_prog_listener: Option<LogProgListenerPair>) -> Result<Arc<FilePathComparisonModel>, Box<dyn std::error::Error + Send>> {
        get_image_path_comparison_xmp_table_analysis(&pool.pool, log_prog_listener).await
            .map(|v| Arc::new(v))
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn get_task_items_from_analysis(&self, _pool: WebServerActionDataContext, analysis: Arc<FilePathComparisonModel>, _log_prog_listener: Option<LogProgListenerPair>) -> Result<HashSet<String>, Box<dyn std::error::Error + Send>> {
        Ok(analysis.files_missing_from_b.clone())
    }

    async fn process_task_item(&self, task_item: String, _dry_run: bool, _pool: WebServerActionDataContext) -> Result<Option<Arc<ImageXmp>>, Box<dyn std::error::Error + Send>> {
        extract_image_xmp_model(&task_item)
            .map(|x| x.map(Arc::new))
    }

    async fn process_task_output(&self, task_output: Arc<ImageXmp>, pool: WebServerActionDataContext) -> Result<(), Box<dyn std::error::Error + Send>> {
        execute_insert_image_xmp_sql(&task_output.image_path, &task_output.xmp, &pool.pool).await
            .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)?;
        Ok(())
    }

    async fn task_already_completed(&self, task_input: &String, pool: WebServerActionDataContext) -> Result<bool, Box<dyn std::error::Error + Send>> {
        query_xmp_table_count(&task_input, &pool.pool).await
            .map(|v| v > 0)
            .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    fn get_description(&self) -> String {
        "if the xmp table is missing any entries, it will add them".to_string()
    }

    fn get_item_name(&self) -> String {
        "xmp".to_string()
    }

    fn get_process_action_name(&self) -> String {
        "add".to_string()
    }
}

pub struct InsertNewXmpOrchestratorAction;
impl InsertNewXmpOrchestratorAction {
    pub fn new() -> AnalysisTaskItemProcessorOrchestrator<Arc<FilePathComparisonModel>, String, HashSet<String>, Arc<ImageXmp>> {
        AnalysisTaskItemProcessorOrchestrator::new(Arc::new(XmpProcessor::new()))
    }
}