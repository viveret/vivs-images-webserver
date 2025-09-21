use async_trait::async_trait;
use convert_case::{Case, Casing};
use nameof::name_of_type;
use sqlx::SqlitePool;
use actix_web::Result;

use crate::actions::action_indicator::{ActionIndicatorCheckMessage, IActionIndicator};
use crate::actions::refresh::update_image_exif::InsertNewImageExifFromDiskAction;
use crate::actions::refresh::update_image_exif::DeleteImageExifFromSqlNotOnDiskAction;
use crate::metrics::exif_metrics::{get_exif_missing_in_sql_count, get_exif_missing_on_disk_count};



pub struct ImagesOnDiskWithMissingExifIndicator {}
impl ImagesOnDiskWithMissingExifIndicator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IActionIndicator for ImagesOnDiskWithMissingExifIndicator {
    fn get_name(&self) -> String {
        name_of_type!(ImagesOnDiskWithMissingExifIndicator).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(ImagesOnDiskWithMissingExifIndicator).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the Exif table is missing any images that are on the disk".to_string()
    }

    fn get_action_name(&self) -> String {
        name_of_type!(InsertNewImageExifFromDiskAction).to_case(Case::Snake)
    }

    fn get_cron_schedule(&self) -> String {
        todo!()
    }

    async fn perform_indicator_check_action(&self, pool: &SqlitePool) -> Result<ActionIndicatorCheckMessage> {
        let (difference_total, msg) = get_exif_missing_in_sql_count(pool).await?;
        Ok(ActionIndicatorCheckMessage(difference_total != 0, msg))
    }
}



pub struct ImagesInExifSqlDbWithMissingImageOnDiskIndicator {}
impl ImagesInExifSqlDbWithMissingImageOnDiskIndicator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IActionIndicator for ImagesInExifSqlDbWithMissingImageOnDiskIndicator {
    fn get_name(&self) -> String {
        name_of_type!(ImagesInExifSqlDbWithMissingImageOnDiskIndicator).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(ImagesInExifSqlDbWithMissingImageOnDiskIndicator).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the Exif table has values for images that are not found or valid on the disk".to_string()
    }

    fn get_action_name(&self) -> String {
        name_of_type!(DeleteImageExifFromSqlNotOnDiskAction).to_case(Case::Snake)
    }

    fn get_cron_schedule(&self) -> String {
        todo!()
    }

    async fn perform_indicator_check_action(&self, pool: &SqlitePool) -> Result<ActionIndicatorCheckMessage> {
        let (difference_total, msg) = get_exif_missing_on_disk_count(pool).await?;
        Ok(ActionIndicatorCheckMessage(difference_total != 0, msg))
    }
}