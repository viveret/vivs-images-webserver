use std::io::ErrorKind;
use std::sync::Arc;

use sqlx::{Pool, Sqlite};

use crate::metrics::thumbnail_metrics::get_image_path_comparison_thumbnail_table_analysis;
use crate::models::image_thumbnail::ImageThumbnail;
use crate::metrics::thumbnail_metrics::ThumbnailMissingAnalysis;
use crate::actions::refresh::analysis_task_item_processor::AnalysisTaskItemProcessorOrchestrator;
use crate::actions::refresh::analysis_task_item_processor::AnalysisTaskItemProcessor;



pub struct ThumbnailProcessor;
impl ThumbnailProcessor {
    pub fn new() -> Self { Self {} }
}

use async_trait::async_trait;


#[async_trait]
impl AnalysisTaskItemProcessor<Arc<ThumbnailMissingAnalysis>, Arc<(String, String)>, Arc<ImageThumbnail>> for ThumbnailProcessor {
    async fn get_analysis(&self, pool: Pool<Sqlite>) -> Result<Arc<ThumbnailMissingAnalysis>, Box<dyn std::error::Error + Send>> {
        get_image_path_comparison_thumbnail_table_analysis(&pool).await
            .map(|v| Arc::new(v))
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn get_task_items_from_analysis(&self, pool: Pool<Sqlite>, analysis: Arc<ThumbnailMissingAnalysis>) -> Result<Vec<Arc<(String, String)>>, Box<dyn std::error::Error + Send>> {
        todo!()
    }

    async fn process_task_item(&self, task_item: Arc<(String, String)>) -> Result<Arc<ImageThumbnail>, Box<dyn std::error::Error + Send>> {
        todo!()
    }

    async fn process_task_output(&self, task_output: Arc<ImageThumbnail>, pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error + Send>> {
        todo!()
    }

    async fn exists_in_db(&self, task_input: &Arc<(String, String)>, pool: Pool<Sqlite>) -> Result<bool, Box<dyn std::error::Error + Send>> {
        todo!()
    }

    fn get_description(&self) -> String {
        "if the thumbnail table is missing any entries, it will add them".to_string()
    }

    fn get_item_name(&self) -> String {
        "thumbnail".to_string()
    }

    fn get_process_action_name(&self) -> String {
        "add".to_string()
    }
}

pub type InsertNewThumbnailsOrchestratorAction = AnalysisTaskItemProcessorOrchestrator<Arc<ThumbnailMissingAnalysis>, Arc<(String, String)>, Arc<ImageThumbnail>>;
impl InsertNewThumbnailsOrchestratorAction {
    pub fn new2() -> Self {
        AnalysisTaskItemProcessorOrchestrator::new(Arc::new(ThumbnailProcessor::new()))
    }
}