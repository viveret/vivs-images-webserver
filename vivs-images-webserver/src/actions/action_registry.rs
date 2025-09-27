use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Sqlite};

use crate::actions::refresh::new_thumbnail_action::InsertNewThumbnailsOrchestratorAction;
use crate::actions::refresh::update_image_brightness::InsertNewImageBrightnessFromDiskAction;
use crate::actions::refresh::update_image_brightness::DeleteImageBrightnessFromSqlNotOnDiskAction;
use crate::actions::channels::TaskToWorkerSender;
use crate::actions::refresh::update_image_exif::DeleteImageExifFromSqlNotOnDiskAction;
use crate::actions::refresh::update_image_exif::InsertNewImageExifFromDiskAction;
use crate::actions::refresh::update_image_similarity::DeleteImageSimilarityFromSqlNotOnDiskAction;
use crate::actions::refresh::update_image_similarity::InsertNewImageSimilarityFromDiskAction;
use crate::actions::refresh::update_image_similarity::InsertNewImageSimilarityFromSqlDbAction;
use crate::actions::refresh::update_image_thumbnail::DeleteImageThumbnailFromSqlNotOnDiskAction;
use crate::actions::refresh::update_image_thumbnail::InsertNewImageThumbnailFromDiskAction;

#[async_trait]
pub trait IWebServerAction: Send + Sync {
    fn get_name(&self) -> String;
    fn get_label(&self) -> String;
    fn get_description(&self) -> String;
    fn get_is_runnable(&self) -> bool;
    fn get_can_dry_run(&self) -> bool;
    async fn run_task(&self, pool: Pool<Sqlite>, send: TaskToWorkerSender, dry_run: bool, task_id: u32) -> actix_web::Result<(), Box<dyn std::error::Error + Send>>;
}

#[derive(Clone)]
pub struct ActionRegistry {
    actions: Arc<std::collections::HashMap<String, Arc<dyn IWebServerAction>>>,
}

impl ActionRegistry {
    pub fn new() -> Self {
        let mut actions = std::collections::HashMap::new();
        
        // Register all actions
        for action in get_all_actions() {
            actions.insert(action.get_name(), action);
        }
        
        Self { actions: Arc::new(actions) }
    }

    pub fn get_action(&self, name: &str) -> Option<Arc<dyn IWebServerAction>> {
        self.actions.get(name).cloned()
    }

    pub fn list_actions(&self) -> Vec<String> {
        self.actions.keys().cloned().collect()
    }

    pub fn get_all_actions(&self) -> Vec<Arc<dyn IWebServerAction>> {
        self.actions.values().into_iter().cloned().collect()
    }
}

// Helper functions
pub fn find_action(name: String) -> Option<Arc<dyn IWebServerAction>> {
    get_all_actions()
        .iter()
        .find(|a| a.get_name() == name)
        .map(Arc::clone)
}

pub fn get_all_actions() -> Vec<Arc<dyn IWebServerAction>> {
    let mut actions: Vec<Arc<dyn IWebServerAction>> = vec![
        Arc::new(InsertNewImageBrightnessFromDiskAction::new()),
        Arc::new(DeleteImageBrightnessFromSqlNotOnDiskAction::new()),
        Arc::new(InsertNewImageExifFromDiskAction::new()),
        Arc::new(DeleteImageExifFromSqlNotOnDiskAction::new()),
        Arc::new(InsertNewImageSimilarityFromDiskAction::new()),
        Arc::new(InsertNewImageSimilarityFromSqlDbAction::new()),
        Arc::new(DeleteImageSimilarityFromSqlNotOnDiskAction::new()),
        Arc::new(InsertNewImageThumbnailFromDiskAction::new()),
        Arc::new(DeleteImageThumbnailFromSqlNotOnDiskAction::new()),
        Arc::new(InsertNewThumbnailsOrchestratorAction::new2()),
    ];
    actions.extend_from_slice(&crate::actions::sql_db_actions::get_sql_db_actions());
    actions
}