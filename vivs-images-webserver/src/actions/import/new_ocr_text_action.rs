// new_ocr_text_action.rs

use std::collections::HashSet;
use std::io::ErrorKind;
use std::sync::Arc;

use async_trait::async_trait;

use crate::core::data_context::WebServerActionDataContext;
use crate::actions::analysis_task_item_processor::LogProgListenerPair;
use crate::calc::file_paths_comparison::FilePathComparisonModel;
use crate::converters::extract_image_ocr_text::extract_image_ocr_text;
use crate::database::query::query_image_ocr_text::query_ocr_text_table_count;
use crate::database::update::update_image_ocr_text::execute_insert_image_ocr_text_sql;
use crate::metrics::ocr_text_metrics::get_image_path_comparison_ocr_text_table_analysis;
use crate::models::image_ocr_text::ImageOcrText;
use crate::actions::analysis_task_item_processor::AnalysisTaskItemProcessorOrchestrator;
use crate::actions::analysis_task_item_processor::AnalysisTaskItemProcessor;



pub struct OcrTextProcessor;
impl OcrTextProcessor {
    pub fn new() -> Self { Self {} }
}


#[async_trait]
impl AnalysisTaskItemProcessor<Arc<FilePathComparisonModel>, String, HashSet<String>, Arc<ImageOcrText>> for OcrTextProcessor {
    async fn get_analysis(&self, pool: WebServerActionDataContext, log_prog_listener: Option<LogProgListenerPair>) -> Result<Arc<FilePathComparisonModel>, Box<dyn std::error::Error + Send>> {
        get_image_path_comparison_ocr_text_table_analysis(&pool.pool, log_prog_listener).await
            .map(|v| Arc::new(v))
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn get_task_items_from_analysis(&self, _pool: WebServerActionDataContext, analysis: Arc<FilePathComparisonModel>, log_prog_listener: Option<LogProgListenerPair>) -> Result<HashSet<String>, Box<dyn std::error::Error + Send>> {
        Ok(analysis.files_missing_from_b.clone())
    }

    async fn process_task_item(&self, task_item: String, _dry_run: bool, _pool: WebServerActionDataContext) -> Result<Arc<ImageOcrText>, Box<dyn std::error::Error + Send>> {
        extract_image_ocr_text(&task_item)
            .map(|ocr_text| ImageOcrText { image_path: task_item, ocr_text })
            .map(Arc::new)
            .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn process_task_output(&self, task_output: Arc<ImageOcrText>, pool: WebServerActionDataContext) -> Result<(), Box<dyn std::error::Error + Send>> {
        execute_insert_image_ocr_text_sql(&task_output, pool.pool).await
            .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn task_already_completed(&self, task_input: &String, pool: WebServerActionDataContext) -> Result<bool, Box<dyn std::error::Error + Send>> {
        query_ocr_text_table_count(&task_input, &pool.pool).await
            .map(|v| v > 0)
            .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    fn get_description(&self) -> String {
        "if the ocr_text table is missing any entries, it will add them".to_string()
    }

    fn get_item_name(&self) -> String {
        "ocr_text".to_string()
    }

    fn get_process_action_name(&self) -> String {
        "add".to_string()
    }
}

pub struct InsertNewOcrTextsOrchestratorAction;
impl InsertNewOcrTextsOrchestratorAction {
    pub fn new() -> AnalysisTaskItemProcessorOrchestrator<Arc<FilePathComparisonModel>, String, HashSet<String>, Arc<ImageOcrText>> {
        AnalysisTaskItemProcessorOrchestrator::new(Arc::new(OcrTextProcessor::new()))
    }
}