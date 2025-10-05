// new_exif_tags_action.rs

use std::collections::HashSet;
use std::sync::Arc;

use async_trait::async_trait;

use crate::converters::extract_image_exif::extract_image_exif_tags;
use crate::converters::extract_image_iptc::extract_image_iptc_tags;
use crate::core::data_context::WebServerActionDataContext;
use crate::actions::analysis_task_item_processor::LogProgListenerPair;
use crate::calc::file_paths_comparison::FilePathComparisonModel;
use crate::database::query::query_image_tag::query_image_tags_table_count_for_path;
use crate::database::update::update_image_tags::execute_insert_image_tag_sql;
use crate::metrics::tag_metrics::get_image_path_comparison_tags_table_files_with_tags_tags_analysis;
use crate::actions::analysis_task_item_processor::AnalysisTaskItemProcessorOrchestrator;
use crate::actions::analysis_task_item_processor::AnalysisTaskItemProcessor;
use crate::models::image_tag::ImageTagSet;



pub struct ImageTagsProcessor;
impl ImageTagsProcessor {
    pub fn new() -> Self { Self {} }
}


#[async_trait]
impl AnalysisTaskItemProcessor<Arc<FilePathComparisonModel>, String, HashSet<String>, Arc<ImageTagSet>> for ImageTagsProcessor {
    async fn get_analysis(&self, pool: WebServerActionDataContext, log_prog_listener: Option<LogProgListenerPair>) -> Result<Arc<FilePathComparisonModel>, Box<dyn std::error::Error + Send>> {
        get_image_path_comparison_tags_table_files_with_tags_tags_analysis(log_prog_listener, &pool.pool).await
            .map(|v| Arc::new(v))
    }

    async fn get_task_items_from_analysis(&self, _pool: WebServerActionDataContext, analysis: Arc<FilePathComparisonModel>, log_prog_listener: Option<LogProgListenerPair>) -> Result<HashSet<String>, Box<dyn std::error::Error + Send>> {
        Ok(analysis.files_missing_from_b.clone())
    }

    async fn process_task_item(&self, task_item: String, _dry_run: bool, _pool: WebServerActionDataContext) -> Result<Arc<ImageTagSet>, Box<dyn std::error::Error + Send>> {
        let mut exif_tags = extract_image_exif_tags(&task_item)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;

        let iptc_tags = if task_item.ends_with(".jpeg") || task_item.ends_with(".jpg") || task_item.ends_with(".tiff") || task_item.ends_with(".tif") {
            extract_image_iptc_tags(&task_item)?
        } else { HashSet::new() };
        
        // combine and return
        for t in iptc_tags {
            exif_tags.insert(t);
        }
        Ok(Arc::new(ImageTagSet::new_from_strings(task_item, exif_tags)))
    }

    async fn process_task_output(&self, task_output: Arc<ImageTagSet>, pool: WebServerActionDataContext) -> Result<(), Box<dyn std::error::Error + Send>> {
        for tag in &task_output.0 {
            execute_insert_image_tag_sql(tag.clone(), &pool.pool).await
                .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)?
        }

        Ok(())
    }

    async fn task_already_completed(&self, task_input: &String, pool: WebServerActionDataContext) -> Result<bool, Box<dyn std::error::Error + Send>> {
        query_image_tags_table_count_for_path(&task_input, &pool.pool).await
            .map(|v| v > 0)
            .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    fn get_description(&self) -> String {
        "if the image tag table is missing any exif or iptc tags from files, it will add them".to_string()
    }

    fn get_item_name(&self) -> String {
        "image_tag".to_string()
    }

    fn get_process_action_name(&self) -> String {
        "add_from_disk".to_string()
    }
}

pub struct InsertNewImageTagsFromDiskAction;
impl InsertNewImageTagsFromDiskAction {
    pub fn new() -> AnalysisTaskItemProcessorOrchestrator<Arc<FilePathComparisonModel>, String, HashSet<String>, Arc<ImageTagSet>> {
        AnalysisTaskItemProcessorOrchestrator::new(Arc::new(ImageTagsProcessor::new()))
    }
}