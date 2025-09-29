// delete_missing_thumbnails_action.rs

use std::io::ErrorKind;
use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

use crate::actions::refresh::analysis_task_item_processor::AnalysisTaskItemProcessor;
use crate::actions::refresh::analysis_task_item_processor::AnalysisTaskItemProcessorOrchestrator;
use crate::calc::file_paths_comparison::FilePathComparisonModel;
use crate::database::query::query_image_thumbnail::query_thumbnail_table_count;
use crate::database::update::update_image_thumbnail::execute_delete_image_thumbnail_sql;
use crate::metrics::thumbnail_metrics::get_image_path_comparison_thumbnail_table_analysis;


pub struct ThumbnailProcessor;
impl ThumbnailProcessor {
    pub fn new() -> Self { Self {} }
}


#[async_trait]
impl AnalysisTaskItemProcessor<Arc<FilePathComparisonModel>, String, String> for ThumbnailProcessor {
    async fn get_analysis(&self, pool: Pool<Sqlite>) -> Result<Arc<FilePathComparisonModel>, Box<dyn std::error::Error + Send>> {
        get_image_path_comparison_thumbnail_table_analysis(&pool).await
            .map(|v| Arc::new(v))
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn get_task_items_from_analysis(&self, _pool: Pool<Sqlite>, analysis: Arc<FilePathComparisonModel>) -> Result<Vec<String>, Box<dyn std::error::Error + Send>> {
        Ok(analysis.files_missing_from_a.clone())
    }

    async fn process_task_item(&self, task_item: String, _pool: Pool<Sqlite>) -> Result<String, Box<dyn std::error::Error + Send>> {
        Ok(task_item)
    }

    async fn process_task_output(&self, task_output: String, pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error + Send>> {
        execute_delete_image_thumbnail_sql(&task_output, &pool).await
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn exists_in_db(&self, task_input: &String, pool: Pool<Sqlite>) -> Result<bool, Box<dyn std::error::Error + Send>> {
        query_thumbnail_table_count(&task_input, &pool).await
            .map(|v| v > 0)
            .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    fn get_description(&self) -> String {
        "if the thumbnail table has any entries missing from disk, it will delete them".to_string()
    }

    fn get_item_name(&self) -> String {
        "thumbnail".to_string()
    }

    fn get_process_action_name(&self) -> String {
        "delete_missing".to_string()
    }
}

pub struct DeleteMissingThumbnailsOrchestratorAction;
impl DeleteMissingThumbnailsOrchestratorAction {
    pub fn new2() -> AnalysisTaskItemProcessorOrchestrator<Arc<FilePathComparisonModel>, String, String> {
        AnalysisTaskItemProcessorOrchestrator::new(Arc::new(ThumbnailProcessor::new()))
    }
}