use async_trait::async_trait;
use convert_case::{Case, Casing};
use nameof::name_of_type;
use sqlx::SqlitePool;
use actix_web::Result;

use crate::actions::action_indicator::{ActionIndicatorCheckMessage, IActionIndicator};
use crate::actions::refresh::update_image_thumbnail::InsertNewImageThumbnailFromDiskAction;
use crate::actions::refresh::update_image_thumbnail::DeleteImageThumbnailFromSqlNotOnDiskAction;
use crate::metrics::thumbnail_metrics::{get_thumbnail_missing_in_sql_count, get_thumbnail_missing_on_disk_count};



pub struct ImagesOnDiskWithMissingThumbnailIndicator {}
impl ImagesOnDiskWithMissingThumbnailIndicator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IActionIndicator for ImagesOnDiskWithMissingThumbnailIndicator {
    fn get_name(&self) -> String {
        name_of_type!(ImagesOnDiskWithMissingThumbnailIndicator).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(ImagesOnDiskWithMissingThumbnailIndicator).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the thumbnail table is missing any images that are on the disk".to_string()
    }

    fn get_action_name(&self) -> String {
        name_of_type!(InsertNewImageThumbnailFromDiskAction).to_case(Case::Snake)
    }

    fn get_cron_schedule(&self) -> String {
        todo!()
    }

    async fn perform_indicator_check_action(&self, pool: &SqlitePool) -> Result<ActionIndicatorCheckMessage> {
        let (difference_total, msg) = get_thumbnail_missing_in_sql_count(pool).await?;
        Ok(ActionIndicatorCheckMessage(difference_total != 0, msg))
    }
}



pub struct ImagesInThumbnailSqlDbWithMissingImageOnDiskIndicator {}
impl ImagesInThumbnailSqlDbWithMissingImageOnDiskIndicator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IActionIndicator for ImagesInThumbnailSqlDbWithMissingImageOnDiskIndicator {
    fn get_name(&self) -> String {
        name_of_type!(ImagesInThumbnailSqlDbWithMissingImageOnDiskIndicator).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(ImagesInThumbnailSqlDbWithMissingImageOnDiskIndicator).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the thumbnail table has entries for images that are not found or valid on the disk".to_string()
    }

    fn get_action_name(&self) -> String {
        name_of_type!(DeleteImageThumbnailFromSqlNotOnDiskAction).to_case(Case::Snake)
    }

    fn get_cron_schedule(&self) -> String {
        todo!()
    }

    async fn perform_indicator_check_action(&self, pool: &SqlitePool) -> Result<ActionIndicatorCheckMessage> {
        let (difference_total, msg) = get_thumbnail_missing_on_disk_count(pool).await?;
        Ok(ActionIndicatorCheckMessage(difference_total != 0, msg))
    }
}