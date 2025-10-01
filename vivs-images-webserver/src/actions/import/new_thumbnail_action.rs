// new_thumbnail_action.rs

use std::collections::HashSet;
use std::io::ErrorKind;
use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

use crate::actions::analysis_task_item_processor::LogProgListenerPair;
use crate::calc::file_paths_comparison::FilePathComparisonModel;
use crate::converters::extract_image_thumbnail::open_and_extract_multiple_image_thumbnails_standard_sizes;
use crate::database::query::query_image_thumbnail::query_thumbnail_table_count;
use crate::database::update::update_image_thumbnail::execute_insert_image_thumbnail_sql;
use crate::metrics::thumbnail_metrics::get_image_path_comparison_thumbnail_table_analysis;
use crate::models::image_thumbnail::ImageThumbnail;
use crate::actions::analysis_task_item_processor::AnalysisTaskItemProcessorOrchestrator;
use crate::actions::analysis_task_item_processor::AnalysisTaskItemProcessor;
use crate::models::image_thumbnail::ImageThumbnailVec;
use crate::models::image_thumbnail::ThumbnailFormat;


pub struct ThumbnailProcessor;
impl ThumbnailProcessor {
    pub fn new() -> Self { Self {} }
}


#[async_trait]
impl AnalysisTaskItemProcessor<Arc<FilePathComparisonModel>, String, HashSet<String>, Arc<ImageThumbnailVec>> for ThumbnailProcessor {
    async fn get_analysis(&self, pool: Pool<Sqlite>, log_prog_listener: Option<LogProgListenerPair>) -> Result<Arc<FilePathComparisonModel>, Box<dyn std::error::Error + Send>> {
        get_image_path_comparison_thumbnail_table_analysis(&pool).await
            .map(|v| Arc::new(v))
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn get_task_items_from_analysis(&self, _pool: Pool<Sqlite>, analysis: Arc<FilePathComparisonModel>, log_prog_listener: Option<LogProgListenerPair>) -> Result<HashSet<String>, Box<dyn std::error::Error + Send>> {
        Ok(analysis.files_missing_from_b.clone())
    }

    async fn process_task_item(&self, task_item: String, _pool: Pool<Sqlite>) -> Result<Arc<ImageThumbnailVec>, Box<dyn std::error::Error + Send>> {
        open_and_extract_multiple_image_thumbnails_standard_sizes(&task_item)
            .map(|vals| {
                let imgs = vals.iter()
                    .map(|img| ImageThumbnail::from_image(task_item.clone(), ThumbnailFormat::PNG, img))
                    .collect::<Vec<ImageThumbnail>>();
                Arc::new(ImageThumbnailVec(imgs))
            }).map_err(|e| {
                Box::new(e) as Box<dyn std::error::Error + Send>
            })
    }

    async fn process_task_output(&self, task_output: Arc<ImageThumbnailVec>, pool: Pool<Sqlite>) -> Result<(), Box<dyn std::error::Error + Send>> {
        for thumbnail in task_output.0.iter() {
            execute_insert_image_thumbnail_sql(&thumbnail, &pool).await
                .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)?;
        }
        Ok(())
    }

    async fn task_already_completed(&self, task_input: &String, pool: Pool<Sqlite>) -> Result<bool, Box<dyn std::error::Error + Send>> {
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

pub struct InsertNewThumbnailsOrchestratorAction;
impl InsertNewThumbnailsOrchestratorAction {
    pub fn new2() -> AnalysisTaskItemProcessorOrchestrator<Arc<FilePathComparisonModel>, String, HashSet<String>, Arc<ImageThumbnailVec>> {
        AnalysisTaskItemProcessorOrchestrator::new(Arc::new(ThumbnailProcessor::new()))
    }
}