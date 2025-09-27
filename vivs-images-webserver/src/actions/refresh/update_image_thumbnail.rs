// update_image_thumbnail.rs

use std::io::ErrorKind;

use async_trait::async_trait;
use convert_case::{Case, Casing};
use nameof::name_of_type;
use sqlx::{Pool, Sqlite};

use crate::actions::channels::TaskToWorkerSender;
use crate::actions::channels::TaskToWorkerMessage;
use crate::actions::channels::task_to_worker_send_helper2;
use crate::actions::action_registry::IWebServerAction;
use crate::converters::extract_image_thumbnail::open_and_extract_multiple_image_thumbnails_standard_sizes;
use crate::database::update::update_image_thumbnail::execute_insert_image_thumbnail_sql;
use crate::database::update::update_image_thumbnail::execute_delete_image_thumbnail_sql;
use crate::metrics::thumbnail_metrics::get_image_path_comparison_thumbnail_table_analysis;
use crate::models::image_thumbnail::{ImageThumbnail, ThumbnailFormat};

pub struct InsertNewImageThumbnailFromDiskAction {}

impl InsertNewImageThumbnailFromDiskAction { pub fn new() -> Self { Self {} } }

#[async_trait]
impl IWebServerAction for InsertNewImageThumbnailFromDiskAction {
    fn get_name(&self) -> String {
        name_of_type!(InsertNewImageThumbnailFromDiskAction).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(InsertNewImageThumbnailFromDiskAction).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the thumbnail table is missing any values it will add them.".to_string()
    }

    fn get_is_runnable(&self) -> bool { true }
    
    fn get_can_dry_run(&self) -> bool { true }

    async fn run_task(&self, pool: Pool<Sqlite>, send: TaskToWorkerSender, dry_run: bool, task_id: u32) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        let analysis = get_image_path_comparison_thumbnail_table_analysis(&pool).await
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)?;
        task_to_worker_send_helper2(&send, TaskToWorkerMessage::LogInfo(task_id, analysis.log))?;
        task_to_worker_send_helper2(&send, TaskToWorkerMessage::LogError(task_id, analysis.log_error))?;
        task_to_worker_send_helper2(&send, TaskToWorkerMessage::LogInfo(task_id, analysis.message))?;

        let mut missing_index = 0;
        let missing_count = analysis.files_missing_from_sql.len();
        for missing_from_db in analysis.files_missing_from_sql {
            let task_result = open_and_extract_multiple_image_thumbnails_standard_sizes(&missing_from_db);
            match task_result {
                Ok(thumbnails) => {
                    for thumbnail in thumbnails {
                        let thumbnail = ImageThumbnail::from_image(missing_from_db.clone(), ThumbnailFormat::PNG, &thumbnail);
                        if dry_run {
                            task_to_worker_send_helper2(&send, TaskToWorkerMessage::LogInfo(task_id, format!("Dry run thumbnail for {}", missing_from_db)))?;
                        } else {
                            match execute_insert_image_thumbnail_sql(&thumbnail, &pool).await {
                                Ok(()) => {
                                    task_to_worker_send_helper2(&send, TaskToWorkerMessage::LogInfo(task_id, format!("Updated thumbnail for {}", missing_from_db)))?;
                                },
                                Err(e) => {
                                    task_to_worker_send_helper2(&send, TaskToWorkerMessage::LogError(task_id, format!("update db image thumbnail error: {}", e)))?;
                                }
                            }
                        }
                    }
                },
                Err(e) => {
                    task_to_worker_send_helper2(&send, TaskToWorkerMessage::LogError(task_id, format!("extract image thumbnail error: {}", e)))?;
                },
            }

            missing_index = missing_index + 1;
            let missing_progress = (missing_index as f64) / (missing_count as f64);
            let new_progress = missing_progress;
            task_to_worker_send_helper2(&send, TaskToWorkerMessage::ProgressUpdate(task_id, new_progress as f32))?;
        }
        
        task_to_worker_send_helper2(&send, TaskToWorkerMessage::ProgressUpdate(task_id, 1.0))?;
        Ok(())
    }
}


pub struct DeleteImageThumbnailFromSqlNotOnDiskAction {}
impl DeleteImageThumbnailFromSqlNotOnDiskAction {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl IWebServerAction for DeleteImageThumbnailFromSqlNotOnDiskAction {
    fn get_name(&self) -> String {
        name_of_type!(DeleteImageThumbnailFromSqlNotOnDiskAction).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(DeleteImageThumbnailFromSqlNotOnDiskAction).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the thumbnail table has any values not found on disk it will delete them.".to_string()
    }

    fn get_is_runnable(&self) -> bool { true }
    
    fn get_can_dry_run(&self) -> bool { true }
    
    async fn run_task(&self, pool: Pool<Sqlite>, send: TaskToWorkerSender, dry_run: bool, task_id: u32) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        let analysis = get_image_path_comparison_thumbnail_table_analysis(&pool).await
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)?;
        
        task_to_worker_send_helper2(&send, TaskToWorkerMessage::LogInfo(task_id, analysis.log))?;
        task_to_worker_send_helper2(&send, TaskToWorkerMessage::LogError(task_id, analysis.log_error))?;
        task_to_worker_send_helper2(&send, TaskToWorkerMessage::LogInfo(task_id, analysis.message))?;

        let mut missing_index = 0;
        let missing_count = analysis.files_missing_from_disk.len();
        for missing_from_disk in analysis.files_missing_from_disk {
            if dry_run {
                task_to_worker_send_helper2(&send, TaskToWorkerMessage::LogInfo(task_id, format!("Dry run {} from db", missing_from_disk)))?;
            } else {
                match execute_delete_image_thumbnail_sql(&missing_from_disk, &pool).await {
                    Ok(()) => {
                        task_to_worker_send_helper2(&send, TaskToWorkerMessage::LogInfo(task_id, format!("Deleted {} from db", missing_from_disk)))?;
                    },
                    Err(e) => {
                        task_to_worker_send_helper2(&send, TaskToWorkerMessage::LogError(task_id, format!("Delete {} from db error: {}", missing_from_disk, e)))?;
                    }
                }
            }

            missing_index = missing_index + 1;
            let missing_progress = (missing_index as f64) / (missing_count as f64);
            let new_progress = missing_progress;
            task_to_worker_send_helper2(&send, TaskToWorkerMessage::ProgressUpdate(task_id, new_progress as f32))?;
        }
        
        task_to_worker_send_helper2(&send, TaskToWorkerMessage::ProgressUpdate(task_id, 1.0))?;
        Ok(())
    }
}