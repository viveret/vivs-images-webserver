use std::{io::ErrorKind, sync::Arc};
use std::collections::HashMap;

use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

use crate::actions::refresh::analysis_task_item_processor::TaskOrchestrationOptions;
use crate::actions::{action_registry::IWebServerAction, channels::TaskToWorkerSender};


pub struct SqlDbAction {
    name: String,
    label: String,
    description: String,
    is_runnable: bool,
    script: String,
}

impl SqlDbAction {
    pub fn new(script: &str) -> Self {
        let metadata = Self::parse_metadata(script);
        Self {
            name: metadata.get("name").cloned().unwrap_or_else(|| "unnamed_action".to_string()),
            label: metadata.get("label").cloned().unwrap_or_else(|| "Unnamed Action".to_string()),
            description: metadata.get("description").cloned().unwrap_or_else(|| "No description".to_string()),
            is_runnable: metadata.get("is_runnable")
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            script: script.to_string(),
        }
    }

    /// Parses metadata from SQL script header comments
    /// Expected format: -- @key: value
    fn parse_metadata(script: &str) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        
        for line in script.lines() {
            let line = line.trim();
            
            // Stop parsing if we encounter non-comment lines
            if !line.starts_with("--") && !line.is_empty() {
                break;
            }
            
            // Remove comment markers and trim
            let line_content = line.trim_start_matches("--").trim();
            
            // Check for key-value format: @key: value
            if let Some(colon_pos) = line_content.find(':') {
                let key = line_content[..colon_pos].trim().trim_start_matches('@').to_string();
                let value = line_content[colon_pos + 1..].trim().to_string();
                
                if !key.is_empty() && !value.is_empty() {
                    metadata.insert(key, value);
                }
            }
        }
        
        metadata
    }

    pub fn to_string(&self) -> String {
        self.script.to_string()
    }
}

#[async_trait]
impl IWebServerAction for SqlDbAction {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_label(&self) -> String {
        self.label.clone()
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_is_runnable(&self) -> bool {
        self.is_runnable
    }
    
    fn get_can_dry_run(&self) -> bool { false }
    
    async fn run_task(&self, _pool: Pool<Sqlite>, send: TaskToWorkerSender, _dry_run: bool, task_id: u32, _orch_options: TaskOrchestrationOptions) -> actix_web::Result<(), Box<dyn std::error::Error + Send>> {
        send.send(super::channels::TaskToWorkerMessage::LogInfo(task_id, format!("Task has been run!")))
            .map_err(|e| Box::new(std::io::Error::new(ErrorKind::Other, format!("{}", e))) as Box<dyn std::error::Error + Send>)?;
        Ok(())
    }
    
}

pub fn get_sql_db_actions() -> Vec<Arc<dyn IWebServerAction>> {
    vec![
        Arc::new(SqlDbAction::new(crate::database::cleanup::clean_image_brightness::CLEAN_IMAGE_BRIGHTNESS_SQL)),
        Arc::new(SqlDbAction::new(crate::database::cleanup::clean_image_exif::CLEAN_IMAGE_EXIF_SQL)),
        Arc::new(SqlDbAction::new(crate::database::cleanup::clean_image_similarity::CLEAN_IMAGE_SIMILARITY_SQL)),
    ]
}