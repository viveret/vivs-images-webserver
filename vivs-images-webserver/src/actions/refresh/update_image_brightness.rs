use async_trait::async_trait;
use convert_case::{Case, Casing};
use nameof::name_of_type;
use sqlx::{Pool, Sqlite};

use crate::actions::channels::{task_to_worker_send_helper, TaskToWorkerMessage, TaskToWorkerSender};
use crate::actions::action_registry::IWebServerAction;
use crate::converters::extract_image_brightness::{extract_image_brightness, ImageToBrightnessAlgo, ImageToBrightnessOptions};
use crate::database::update::update_image_brightness::{execute_delete_image_brightness_sql, execute_insert_image_brightness_sql};
use crate::metrics::brightness_metrics::{get_image_path_comparison_analysis, BrightnessMissingAnalysis};

pub struct InsertNewImageBrightnessFromDiskAction {}

impl InsertNewImageBrightnessFromDiskAction {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IWebServerAction for InsertNewImageBrightnessFromDiskAction {
    fn get_name(&self) -> String {
        name_of_type!(InsertNewImageBrightnessFromDiskAction).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(InsertNewImageBrightnessFromDiskAction).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the brightness table is missing any values it will add them.".to_string()
    }

    fn get_is_runnable(&self) -> bool {
        true
    }
    
    fn get_can_dry_run(&self) -> bool { false }

    async fn run_task(&self, pool: Pool<Sqlite>, send: TaskToWorkerSender, dry_run: bool, task_id: u32) -> actix_web::Result<()> {
        // first 1/3 of progress is getting the difference list
        let analysis = get_image_path_comparison_analysis(&pool).await?;
        let analysis: BrightnessMissingAnalysis = (*analysis).clone();
        task_to_worker_send_helper(&send, TaskToWorkerMessage::LogInfo(task_id, analysis.log))?;
        task_to_worker_send_helper(&send, TaskToWorkerMessage::LogError(task_id, analysis.log_error))?;
        task_to_worker_send_helper(&send, TaskToWorkerMessage::LogInfo(task_id, analysis.message))?;
        task_to_worker_send_helper(&send, TaskToWorkerMessage::ProgressUpdate(task_id, 0.3))?;

        // last 2/3 of progress are calculating the values and updating the db
        let mut missing_index = 0;
        let missing_count = analysis.files_missing_from_sql.len();
        for missing_from_db in analysis.files_missing_from_sql {
            let options = ImageToBrightnessOptions {
                    algo: ImageToBrightnessAlgo::SimpleImageRS
            };
            let task_result = extract_image_brightness(&missing_from_db, &options);
            match task_result {
                Ok(brightness) => {
                    match execute_insert_image_brightness_sql(&missing_from_db, brightness, &pool).await {
                        Ok(()) => {
                            task_to_worker_send_helper(&send, TaskToWorkerMessage::LogInfo(task_id, format!("Updated brightness to {} for {}", brightness, missing_from_db)))?;
                        },
                        Err(e) => {
                            task_to_worker_send_helper(&send, TaskToWorkerMessage::LogError(task_id, format!("update db image brightness error: {}", e)))?;
                        }
                    }
                },
                Err(e) => {
                    task_to_worker_send_helper(&send, TaskToWorkerMessage::LogError(task_id, format!("extract image brightness error: {}", e)))?;
                },
            }

            missing_index = missing_index + 1;
            let missing_progress = (missing_index as f64) / (missing_count as f64);
            let new_progress = 0.3 + missing_progress * 0.7;
            task_to_worker_send_helper(&send, TaskToWorkerMessage::ProgressUpdate(task_id, new_progress as f32))?;
        }
        
        task_to_worker_send_helper(&send, TaskToWorkerMessage::ProgressUpdate(task_id, 1.0))?;
        Ok(())
    }
}


pub struct DeleteImageBrightnessFromSqlNotOnDiskAction {}
impl DeleteImageBrightnessFromSqlNotOnDiskAction {
    pub fn new() -> Self { Self {} }
}

#[async_trait]
impl IWebServerAction for DeleteImageBrightnessFromSqlNotOnDiskAction {
    fn get_name(&self) -> String {
        name_of_type!(DeleteImageBrightnessFromSqlNotOnDiskAction).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(DeleteImageBrightnessFromSqlNotOnDiskAction).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the brightness table has any values not found on disk it will delete them.".to_string()
    }

    fn get_is_runnable(&self) -> bool {
        true
    }
    
    fn get_can_dry_run(&self) -> bool { false }
    
    async fn run_task(&self, pool: Pool<Sqlite>, send: TaskToWorkerSender, dry_run: bool, task_id: u32) -> actix_web::Result<()> {
        // first 1/3 of progress is getting the difference list
        let analysis = get_image_path_comparison_analysis(&pool).await?;
        let analysis: BrightnessMissingAnalysis = (*analysis).clone();
        task_to_worker_send_helper(&send, TaskToWorkerMessage::LogInfo(task_id, analysis.log))?;
        task_to_worker_send_helper(&send, TaskToWorkerMessage::LogError(task_id, analysis.log_error))?;
        task_to_worker_send_helper(&send, TaskToWorkerMessage::LogInfo(task_id, analysis.message))?;
        task_to_worker_send_helper(&send, TaskToWorkerMessage::ProgressUpdate(task_id, 0.3))?;

        // last 2/3 of progress are deleting rows
        let mut missing_index = 0;
        let missing_count = analysis.files_missing_from_disk.len();
        for missing_from_disk in analysis.files_missing_from_disk {
            match execute_delete_image_brightness_sql(&missing_from_disk, &pool).await {
                Ok(()) => {
                    task_to_worker_send_helper(&send, TaskToWorkerMessage::LogInfo(task_id, format!("Deleted {} from db", missing_from_disk)))?;
                },
                Err(e) => {
                    task_to_worker_send_helper(&send, TaskToWorkerMessage::LogError(task_id, format!("Delete {} from db error: {}", missing_from_disk, e)))?;
                }
            }

            missing_index = missing_index + 1;
            let missing_progress = (missing_index as f64) / (missing_count as f64);
            let new_progress = 0.3 + missing_progress * 0.7;
            task_to_worker_send_helper(&send, TaskToWorkerMessage::ProgressUpdate(task_id, new_progress as f32))?;
        }
        
        task_to_worker_send_helper(&send, TaskToWorkerMessage::ProgressUpdate(task_id, 1.0))?;
        Ok(())
    }
}