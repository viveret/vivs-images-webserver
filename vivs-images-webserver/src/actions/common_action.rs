// use std::io::ErrorKind;

// use async_trait::async_trait;
// use convert_case::{Case, Casing};
// use nameof::name_of_type;
// use sqlx::{Pool, Sqlite};

// use crate::actions::channels::{task_to_worker_send_helper, TaskToWorkerMessage, TaskToWorkerSender};
// use crate::actions::action_registry::IWebServerAction;

// // Generic trait for data processing operations
// #[async_trait]
// pub trait DataProcessor<T, E> {
//     type Analysis;
//     type Input;
//     type Output;
    
//     async fn get_analysis(&self, pool: &Pool<Sqlite>) -> Result<Self::Analysis, E>;
//     async fn process_item(&self, input: Self::Input, pool: &Pool<Sqlite>) -> Result<Self::Output, E>;
//     async fn insert_data(&self, output: Self::Output, pool: &Pool<Sqlite>) -> Result<(), E>;
//     async fn delete_data(&self, identifier: &str, pool: &Pool<Sqlite>) -> Result<(), E>;
// }

// // Generic action handler for common patterns
// pub struct GenericActionHandler<P, T, E> 
// where 
//     P: DataProcessor<T, E> + Send + Sync,
//     E: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static,
// {
//     processor: P,
//     action_type: ActionType,
//     dry_run_supported: bool,
//     _phantom: std::marker::PhantomData<(T, E)>,
// }

// #[derive(Clone)]
// pub enum ActionType {
//     InsertFromDisk,
//     DeleteFromSqlNotOnDisk,
// }

// impl<P, T, E> GenericActionHandler<P, T, E>
// where 
//     P: DataProcessor<T, E> + Send + Sync,
//     E: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static,
// {
//     pub fn new(processor: P, action_type: ActionType, dry_run_supported: bool) -> Self {
//         Self {
//             processor,
//             action_type,
//             dry_run_supported,
//             _phantom: std::marker::PhantomData,
//         }
//     }

//     pub async fn run_generic_task(
//         &self,
//         pool: Pool<Sqlite>,
//         send: TaskToWorkerSender,
//         dry_run: bool,
//         task_id: u32,
//     ) -> actix_web::Result<()> {
//         let analysis = self.processor.get_analysis(&pool).await
//             .map_err(|e| std::io::Error::ErrorInternalServerError(e))?;
        
//         self.send_analysis_logs(&send, task_id, &analysis).await?;
        
//         match self.action_type {
//             ActionType::InsertFromDisk => {
//                 self.handle_insert_action(pool, send, dry_run, task_id, analysis).await
//             }
//             ActionType::DeleteFromSqlNotOnDisk => {
//                 self.handle_delete_action(pool, send, dry_run, task_id, analysis).await
//             }
//         }
//     }

//     async fn handle_insert_action(
//         &self,
//         pool: Pool<Sqlite>,
//         send: TaskToWorkerSender,
//         dry_run: bool,
//         task_id: u32,
//         analysis: P::Analysis,
//     ) -> actix_web::Result<()> {
//         let items = self.get_missing_items_for_insert(&analysis);
//         let total_items = items.len();
        
//         for (index, item) in items.into_iter().enumerate() {
//             match self.processor.process_item(item, &pool).await {
//                 Ok(output) => {
//                     if dry_run {
//                         self.send_log_info(&send, task_id, 
//                             format!("Dry run: Would process item"))?;
//                     } else {
//                         match self.processor.insert_data(output, &pool).await {
//                             Ok(()) => {
//                                 self.send_log_info(&send, task_id, 
//                                     format!("Successfully processed item"))?;
//                             }
//                             Err(e) => {
//                                 self.send_log_error(&send, task_id, 
//                                     format!("Database insert error: {}", e))?;
//                             }
//                         }
//                     }
//                 }
//                 Err(e) => {
//                     self.send_log_error(&send, task_id, 
//                         format!("Processing error: {}", e))?;
//                 }
//             }
            
//             let progress = (index as f32 + 1.0) / total_items as f32;
//             self.send_progress_update(&send, task_id, progress)?;
//         }
        
//         self.send_progress_update(&send, task_id, 1.0)?;
//         Ok(())
//     }

//     async fn handle_delete_action(
//         &self,
//         pool: Pool<Sqlite>,
//         send: TaskToWorkerSender,
//         dry_run: bool,
//         task_id: u32,
//         analysis: P::Analysis,
//     ) -> actix_web::Result<()> {
//         let items = self.get_items_for_delete(&analysis);
//         let total_items = items.len();
        
//         for (index, identifier) in items.into_iter().enumerate() {
//             if dry_run {
//                 self.send_log_info(&send, task_id, 
//                     format!("Dry run: Would delete {}", identifier))?;
//             } else {
//                 match self.processor.delete_data(&identifier, &pool).await {
//                     Ok(()) => {
//                         self.send_log_info(&send, task_id, 
//                             format!("Deleted {}", identifier))?;
//                     }
//                     Err(e) => {
//                         self.send_log_error(&send, task_id, 
//                             format!("Delete error for {}: {}", identifier, e))?;
//                     }
//                 }
//             }
            
//             let progress = (index as f32 + 1.0) / total_items as f32;
//             self.send_progress_update(&send, task_id, progress)?;
//         }
        
//         self.send_progress_update(&send, task_id, 1.0)?;
//         Ok(())
//     }

//     // Helper methods - these would be implemented based on the specific analysis type
//     fn get_missing_items_for_insert(&self, analysis: &P::Analysis) -> Vec<P::Input> {
//         // Default implementation - should be overridden by concrete types
//         vec![]
//     }

//     fn get_items_for_delete(&self, analysis: &P::Analysis) -> Vec<String> {
//         // Default implementation - should be overridden by concrete types
//         vec![]
//     }

//     async fn send_analysis_logs(
//         &self, 
//         send: &TaskToWorkerSender, 
//         task_id: u32, 
//         analysis: &P::Analysis
//     ) -> actix_web::Result<()> {
//         // Default implementation - should be overridden
//         Ok(())
//     }

//     // Common message sending helpers
//     fn send_log_info(&self, send: &TaskToWorkerSender, task_id: u32, message: String) -> actix_web::Result<()> {
//         task_to_worker_send_helper(send, TaskToWorkerMessage::LogInfo(task_id, message))
//     }

//     fn send_log_error(&self, send: &TaskToWorkerSender, task_id: u32, message: String) -> actix_web::Result<()> {
//         task_to_worker_send_helper(send, TaskToWorkerMessage::LogError(task_id, message))
//     }

//     fn send_progress_update(&self, send: &TaskToWorkerSender, task_id: u32, progress: f32) -> actix_web::Result<()> {
//         task_to_worker_send_helper(send, TaskToWorkerMessage::ProgressUpdate(task_id, progress))
//     }
// }

// // Macro to simplify creating action implementations
// macro_rules! create_action {
//     ($action_name:ident, $processor_type:ty, $action_type:expr, $dry_run:expr) => {
//         pub struct $action_name {
//             handler: GenericActionHandler<$processor_type, (), std::io::Error>,
//         }

//         impl $action_name {
//             pub fn new(processor: $processor_type) -> Self {
//                 Self {
//                     handler: GenericActionHandler::new(processor, $action_type, $dry_run),
//                 }
//             }
//         }

//         #[async_trait]
//         impl IWebServerAction for $action_name {
//             fn get_name(&self) -> String {
//                 name_of_type!($action_name).to_case(Case::Snake)
//             }

//             fn get_label(&self) -> String {
//                 name_of_type!($action_name).to_case(Case::Sentence)
//             }

//             fn get_description(&self) -> String {
//                 match $action_type {
//                     ActionType::InsertFromDisk => "If the table is missing any values it will add them.".to_string(),
//                     ActionType::DeleteFromSqlNotOnDisk => "If the table has any values not found on disk it will delete them.".to_string(),
//                 }
//             }

//             fn get_is_runnable(&self) -> bool {
//                 true
//             }

//             fn get_can_dry_run(&self) -> bool {
//                 $dry_run
//             }

//             async fn run_task(&self, pool: Pool<Sqlite>, send: TaskToWorkerSender, dry_run: bool, task_id: u32) -> actix_web::Result<()> {
//                 self.handler.run_generic_task(pool, send, dry_run, task_id).await
//             }
//         }
//     };
// }


// pub struct ThumbnailProcessor;

// #[async_trait]
// impl DataProcessor<(), std::io::Error> for ThumbnailProcessor {
//     type Analysis = crate::metrics::thumbnail_metrics::ThumbnailTableAnalysis;
//     type Input = String;
//     type Output = Vec<crate::models::image_thumbnail::ImageThumbnail>;

//     async fn get_analysis(&self, pool: &Pool<Sqlite>) -> Result<Self::Analysis, std::io::Error> {
//         crate::metrics::thumbnail_metrics::get_image_path_comparison_thumbnail_table_analysis(pool).await
//     }

//     async fn process_item(&self, input: Self::Input, _pool: &Pool<Sqlite>) -> Result<Self::Output, std::io::Error> {
//         crate::converters::extract_image_thumbnail::open_and_extract_multiple_image_thumbnails_standard_sizes(&input)
//             .map_err(|e| std::io::Error::new(ErrorKind::Other, e))
//     }

//     async fn insert_data(&self, output: Self::Output, pool: &Pool<Sqlite>) -> Result<(), std::io::Error> {
//         for thumbnail in output {
//             crate::database::update::update_image_thumbnail::execute_insert_image_thumbnail_sql(&thumbnail, pool).await
//                 .map_err(|e| std::io::Error::ErrorInternalServerError(e))?;
//         }
//         Ok(())
//     }

//     async fn delete_data(&self, identifier: &str, pool: &Pool<Sqlite>) -> Result<(), std::io::Error> {
//         crate::database::update::update_image_thumbnail::execute_delete_image_thumbnail_sql(identifier, pool).await
//             .map_err(|e| std::io::Error::ErrorInternalServerError(e))
//     }
// }

// // Create thumbnail actions using the macro
// create_action!(InsertNewImageThumbnailFromDiskAction, ThumbnailProcessor, ActionType::InsertFromDisk, true);
// create_action!(DeleteImageThumbnailFromSqlNotOnDiskAction, ThumbnailProcessor, ActionType::DeleteFromSqlNotOnDisk, true);