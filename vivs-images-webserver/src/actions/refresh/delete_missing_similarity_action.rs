// delete_missing_similarity_action.rs

use std::collections::HashSet;
use std::io::ErrorKind;
use std::sync::Arc;

use async_trait::async_trait;

use crate::core::data_context::WebServerActionDataContext;
use crate::actions::analysis_task_item_processor::AnalysisTaskItemProcessor;
use crate::actions::analysis_task_item_processor::AnalysisTaskItemProcessorOrchestrator;
use crate::actions::analysis_task_item_processor::LogProgListenerPair;
use crate::calc::file_paths_comparison::FilePathComparisonModel;
use crate::database::query::query_image_similarity::query_similarity_table_count;
use crate::database::update::update_image_similarity::execute_delete_image_similarity_sql;
use crate::metrics::similarity_metrics::get_image_paths_simple_difference_similarity_analysis;


pub struct SimilarityProcessor;
impl SimilarityProcessor {
    pub fn new() -> Self { Self {} }
}


#[async_trait]
impl AnalysisTaskItemProcessor<Arc<FilePathComparisonModel>, String, HashSet<String>, String> for SimilarityProcessor {
    async fn get_analysis(&self, pool: WebServerActionDataContext, log_prog_listener: Option<LogProgListenerPair>) -> Result<Arc<FilePathComparisonModel>, Box<dyn std::error::Error + Send>> {
        get_image_paths_simple_difference_similarity_analysis(&pool.pool).await
            .map(|v| Arc::new(v))
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn get_task_items_from_analysis(&self, _pool: WebServerActionDataContext, analysis: Arc<FilePathComparisonModel>, log_prog_listener: Option<LogProgListenerPair>) -> Result<HashSet<String>, Box<dyn std::error::Error + Send>> {
        Ok(analysis.files_missing_from_b.clone())
    }

    async fn process_task_item(&self, task_item: String, _dry_run: bool, _pool: WebServerActionDataContext) -> Result<String, Box<dyn std::error::Error + Send>> {
        Ok(task_item)
    }

    async fn process_task_output(&self, task_output: String, pool: WebServerActionDataContext) -> Result<(), Box<dyn std::error::Error + Send>> {
        execute_delete_image_similarity_sql(&task_output, &pool.pool).await
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn task_already_completed(&self, task_input: &String, pool: WebServerActionDataContext) -> Result<bool, Box<dyn std::error::Error + Send>> {
        query_similarity_table_count(&task_input, &pool.pool).await
            .map(|v| v > 0)
            .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    fn get_description(&self) -> String {
        "if the similarity table has any entries missing from disk, it will delete them".to_string()
    }

    fn get_item_name(&self) -> String {
        "similarity".to_string()
    }

    fn get_process_action_name(&self) -> String {
        "delete_missing".to_string()
    }
}

pub struct DeleteMissingSimilarityOrchestratorAction;
impl DeleteMissingSimilarityOrchestratorAction {
    pub fn new() -> AnalysisTaskItemProcessorOrchestrator<Arc<FilePathComparisonModel>, String, HashSet<String>, String> {
        AnalysisTaskItemProcessorOrchestrator::new(Arc::new(SimilarityProcessor::new()))
    }
}