use std::sync::Arc;

use async_trait::async_trait;

use crate::actions::analysis_task_item_processor::TaskOrchestrationOptions;
use crate::actions::export::export_image_ocr_text_to_special_dir_action::ExportOcrTextsOrchestratorAction;
use crate::actions::import::calc_aspect_ratio_action::InsertNewAspectRatioOrchestratorAction;
use crate::actions::import::new_image_paths_action::InsertNewImagePathsAction;
use crate::actions::import::new_iptc_action::InsertNewIptcsOrchestratorAction;
use crate::actions::import::new_tags_action::InsertNewImageTagsFromDiskAction;
use crate::actions::import::new_xmp_action::InsertNewXmpOrchestratorAction;
use crate::actions::refresh::delete_missing_brightness_action::DeleteMissingBrightnessOrchestratorAction;
use crate::actions::refresh::delete_missing_exif_action::DeleteMissingExifOrchestratorAction;
use crate::actions::refresh::delete_missing_similarity_action::DeleteMissingSimilarityOrchestratorAction;
use crate::actions::refresh::delete_missing_thumbnails_action::DeleteMissingThumbnailsOrchestratorAction;
use crate::actions::import::new_brightness_action::InsertNewBrightnessOrchestratorAction;
use crate::actions::import::new_exif_action::InsertNewExifsOrchestratorAction;
use crate::actions::import::new_ocr_text_action::InsertNewOcrTextsOrchestratorAction;
use crate::actions::import::new_similarity_action::{InsertNewSimilaritysFromDiskOrchestratorAction, InsertNewSimilaritysFromThumbnailsOrchestratorAction};
use crate::actions::import::new_thumbnail_action::InsertNewThumbnailsOrchestratorAction;
use crate::actions::channels::TaskToWorkerSender;
use crate::core::data_context::WebServerActionDataContext;



#[async_trait]
pub trait IWebServerAction: Send + Sync {
    fn get_name(&self) -> String;
    fn get_label(&self) -> String;
    fn get_description(&self) -> String;
    fn get_is_runnable(&self) -> bool;
    fn get_can_dry_run(&self) -> bool;
    async fn run_task(&self, 
        pool: WebServerActionDataContext, 
        send: TaskToWorkerSender, 
        dry_run: bool, 
        task_id: u32,
        orch_options: TaskOrchestrationOptions
    ) -> actix_web::Result<(), Box<dyn std::error::Error + Send>>;
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
        Arc::new(InsertNewImagePathsAction::new()),
        Arc::new(InsertNewBrightnessOrchestratorAction::new()),
        Arc::new(DeleteMissingBrightnessOrchestratorAction::new()),
        Arc::new(InsertNewExifsOrchestratorAction::new()),
        Arc::new(DeleteMissingExifOrchestratorAction::new()),
        Arc::new(InsertNewSimilaritysFromDiskOrchestratorAction::new()),
        Arc::new(InsertNewSimilaritysFromThumbnailsOrchestratorAction::new()),
        Arc::new(DeleteMissingSimilarityOrchestratorAction::new()),
        Arc::new(InsertNewThumbnailsOrchestratorAction::new()),
        Arc::new(DeleteMissingThumbnailsOrchestratorAction::new()),
        Arc::new(InsertNewOcrTextsOrchestratorAction::new()),
        Arc::new(ExportOcrTextsOrchestratorAction::new()),
        Arc::new(InsertNewAspectRatioOrchestratorAction::new()),
        Arc::new(InsertNewIptcsOrchestratorAction::new()),
        Arc::new(InsertNewImageTagsFromDiskAction::new()),
        Arc::new(InsertNewXmpOrchestratorAction::new()),
        // Arc::new(DeleteMissingAspectRatioOrchestratorAction::new()),
    ];
    actions.extend_from_slice(&crate::actions::sql_db_actions::get_sql_db_actions());
    actions
}