// export_image_ocr_text_to_special_dir_action.rs

use std::io::ErrorKind;
use std::sync::Arc;

use async_trait::async_trait;
use sqlx::SqlitePool;
use sqlx::{Pool, Sqlite};

use crate::actions::analysis_task_item_processor::LogProgListenerPair;
use crate::calc::file_paths_comparison::FilePathComparisonModel;
use crate::database::query::query_image_ocr_text::query_ocr_text_from_db;
use crate::filesystem::query::images::{change_base_path, get_image_ocr_text_export_path, get_photo_sync_path};
use crate::metrics::ocr_text_metrics::get_ocr_text_file_path_comparison_ocr_text_table_analysis;
use crate::models::image_ocr_text::ImageOcrText;
use crate::actions::analysis_task_item_processor::AnalysisTaskItemProcessorOrchestrator;
use crate::actions::analysis_task_item_processor::AnalysisTaskItemProcessor;



pub struct ExportOcrTextProcessor;
impl ExportOcrTextProcessor {
    pub fn new() -> Self { Self {} }
}


pub fn add_extension(path: &String, ext: &str) -> String {
    format!("{}.{}", path, ext)
}

pub fn trim_extension(path: &String, ext: &str) -> String {
    if path.ends_with(ext) {
        path[0..path.len() - ext.len()].to_string()
    } else {
        path.to_string()
    }
}

pub fn trim_extension_txt(path: String) -> String {
    trim_extension(&path, "txt")
}

pub fn add_extension_txt(path: String) -> String {
    add_extension(&path, "txt")
}

pub fn change_image_to_ocr_text_base_path(image_path: &String) -> Result<Option<String>, Box<dyn std::error::Error + Send>> {
    let old_base = get_photo_sync_path()
        .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)?;
    let new_base = get_image_ocr_text_export_path()
        .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)?;

    Ok(change_base_path(image_path, &old_base, &new_base).map(add_extension_txt))
}

pub fn change_ocr_text_to_image_base_path(ocr_text_path: &String) -> Result<Option<String>, Box<dyn std::error::Error + Send>> {
    let old_base = get_image_ocr_text_export_path()
        .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)?;
    let new_base = get_photo_sync_path()
        .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)?;

    Ok(change_base_path(ocr_text_path, &old_base, &new_base).map(trim_extension_txt))
}


#[async_trait]
impl AnalysisTaskItemProcessor<Arc<FilePathComparisonModel>, ImageOcrText, Vec<ImageOcrText>, Arc<String>> for ExportOcrTextProcessor {
    async fn get_analysis(&self, pool: Pool<Sqlite>, log_prog_listener: Option<LogProgListenerPair>) -> Result<Arc<FilePathComparisonModel>, Box<dyn std::error::Error + Send>> {
        get_ocr_text_file_path_comparison_ocr_text_table_analysis(&pool, log_prog_listener).await
            .map(|v| Arc::new(v))
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)
    }

    async fn get_task_items_from_analysis(&self, pool: Pool<Sqlite>, analysis: Arc<FilePathComparisonModel>, _log_prog_listener: Option<LogProgListenerPair>) -> Result<Vec<ImageOcrText>, Box<dyn std::error::Error + Send>> {
        let mut items = vec![];
        for path in analysis.files_missing_from_b.iter() {
            if let Some(path) = change_ocr_text_to_image_base_path(path)? {
                let item = query_ocr_text_from_db(&path, &pool).await
                    .map_err(|e| Box::new(std::io::Error::other(format!("{}", e))) as Box<dyn std::error::Error + Send>)?;
                if let Some(item) = item {
                    items.push(item);
                }
            }
        }
        Ok(items)
    }

    async fn process_task_item(&self, task_item: ImageOcrText, _pool: Pool<Sqlite>) -> Result<Arc<String>, Box<dyn std::error::Error + Send>> {
        if !task_item.ocr_text.is_empty() {
            if let Some(export_to_path) = change_image_to_ocr_text_base_path(&task_item.image_path)? {
                let parent_dir = std::path::Path::new(&export_to_path).parent().unwrap();
                if !parent_dir.exists() {
                    std::fs::create_dir_all(parent_dir)
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;
                }
                std::fs::write(export_to_path, task_item.ocr_text)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)?;
            }
        }
        Ok(Arc::new(String::default()))
    }

    async fn process_task_output(&self, _task_output: Arc<String>, _pool: SqlitePool) -> Result<(), Box<dyn std::error::Error + Send>> {
        Ok(())
    }

    async fn task_already_completed(&self, task_input: &ImageOcrText, _pool: Pool<Sqlite>) -> Result<bool, Box<dyn std::error::Error + Send>> {
        if task_input.ocr_text.is_empty() {
            return Ok(true);
        }

        if let Some(path) = change_image_to_ocr_text_base_path(&task_input.image_path)? {
            std::fs::exists(path)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send>)
        } else {
            Ok(true) // error
        }
    }

    fn get_description(&self) -> String {
        "if the documents folder is missing any ocr text files missing from sql, it will export them".to_string()
    }

    fn get_item_name(&self) -> String {
        "ocr_text".to_string()
    }

    fn get_process_action_name(&self) -> String {
        "export_disk".to_string()
    }
}

pub struct ExportOcrTextsOrchestratorAction;
impl ExportOcrTextsOrchestratorAction {
    pub fn new2() -> AnalysisTaskItemProcessorOrchestrator<Arc<FilePathComparisonModel>, ImageOcrText, Vec<ImageOcrText>, Arc<String>> {
        AnalysisTaskItemProcessorOrchestrator::new(Arc::new(ExportOcrTextProcessor::new()))
    }
}