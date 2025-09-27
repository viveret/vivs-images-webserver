use std::io::ErrorKind;
use std::sync::Arc;

use sqlx::{Pool, Sqlite};

use crate::converters::extract_image_thumbnail::open_and_extract_multiple_image_thumbnails_standard_sizes;
use crate::database::query::query_image_thumbnail::query_thumbnail_table_count;
use crate::database::update::update_image_thumbnail::execute_insert_image_thumbnail_sql;
use crate::metrics::thumbnail_metrics::get_image_path_comparison_thumbnail_table_analysis;
use crate::models::image_thumbnail::ImageThumbnail;
use crate::metrics::thumbnail_metrics::ThumbnailMissingAnalysis;
use crate::actions::refresh::analysis_task_item_processor::AnalysisTaskItemProcessorOrchestrator;
use crate::actions::refresh::analysis_task_item_processor::AnalysisTaskItemProcessor;
use crate::models::image_thumbnail::ThumbnailFormat;



pub struct ThumbnailProcessor;
impl ThumbnailProcessor {
    pub fn new() -> Self { Self {} }
}

use async_trait::async_trait;


#[async_trait]
impl AnalysisTaskItemProcessor<Arc<ThumbnailMissingAnalysis>, String, Arc<Vec<ImageThumbnail>>> for ThumbnailProcessor {
    async fn get_analysis(&self, pool: Pool<Sqlite>) -> Result<Arc<ThumbnailMissingAnalysis>, Box<dyn std::error::Error + Send>> {
        get_image_path_comparison_thumbnail_table_analysis(&pool).await
            .map(|v| Arc::new(v))
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn get_task_items_from_analysis(&self, _pool: Pool<Sqlite>, analysis: Arc<ThumbnailMissingAnalysis>) -> Result<Vec<String>, Box<dyn std::error::Error + Send>> {
        Ok(analysis.files_missing_from_sql.clone())
    }

    async fn process_task_item(&self, task_item: String) -> Result<Arc<Vec<ImageThumbnail>>, Box<dyn std::error::Error + Send>> {
        open_and_extract_multiple_image_thumbnails_standard_sizes(&task_item)
            .map(|vals| {
                let imgs = vals.iter()
                    .map(|img| ImageThumbnail::from_image(task_item.clone(), ThumbnailFormat::PNG, img))
                    .collect::<Vec<ImageThumbnail>>();
                Arc::new(imgs)
            }).map_err(|e| {
                Box::new(e) as Box<dyn std::error::Error + Send>
            })
    }

    async fn process_task_output(&self, task_output: Arc<Vec<ImageThumbnail>>, pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error + Send>> {
        for thumbnail in task_output.iter() {
            execute_insert_image_thumbnail_sql(&thumbnail, &pool).await
                .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)?;
        }
        Ok(())
    }

    async fn exists_in_db(&self, task_input: &String, pool: Pool<Sqlite>) -> Result<bool, Box<dyn std::error::Error + Send>> {
        query_thumbnail_table_count(&task_input, &pool).await
            .map(|v| v > 0)
            .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)
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

pub type InsertNewThumbnailsOrchestratorAction = AnalysisTaskItemProcessorOrchestrator<Arc<ThumbnailMissingAnalysis>, String, Arc<Vec<ImageThumbnail>>>;
impl InsertNewThumbnailsOrchestratorAction {
    pub fn new2() -> Self {
        AnalysisTaskItemProcessorOrchestrator::new(Arc::new(ThumbnailProcessor::new()))
    }
}