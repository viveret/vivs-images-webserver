use async_trait::async_trait;
use convert_case::{Case, Casing};
use nameof::name_of_type;
use sqlx::SqlitePool;
use actix_web::Result;

use crate::actions::action_indicator::{ActionIndicatorCheckMessage, IActionIndicator};
use crate::metrics::brightness_metrics::{get_brightness_missing_in_sql_count, get_brightness_missing_on_disk_count};



pub struct ImagesOnDiskWithMissingBrightnessIndicator {}
impl ImagesOnDiskWithMissingBrightnessIndicator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IActionIndicator for ImagesOnDiskWithMissingBrightnessIndicator {
    fn get_name(&self) -> String {
        name_of_type!(ImagesOnDiskWithMissingBrightnessIndicator).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(ImagesOnDiskWithMissingBrightnessIndicator).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the brightness table is missing any images that are on the disk".to_string()
    }

    fn get_action_name(&self) -> String {
        "add_brightness".to_string()
    }

    fn get_cron_schedule(&self) -> String {
        todo!()
    }

    async fn perform_indicator_check_action(&self, pool: &SqlitePool) -> Result<ActionIndicatorCheckMessage> {
        let (difference_total, msg) = get_brightness_missing_in_sql_count(pool).await?;
        Ok(ActionIndicatorCheckMessage(difference_total != 0, msg))
    }
}



pub struct ImagesInBrightnessSqlDbWithMissingImageOnDiskIndicator {}
impl ImagesInBrightnessSqlDbWithMissingImageOnDiskIndicator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IActionIndicator for ImagesInBrightnessSqlDbWithMissingImageOnDiskIndicator {
    fn get_name(&self) -> String {
        name_of_type!(ImagesInBrightnessSqlDbWithMissingImageOnDiskIndicator).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(ImagesInBrightnessSqlDbWithMissingImageOnDiskIndicator).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the brightness table has values for images that are not found or valid on the disk".to_string()
    }

    fn get_action_name(&self) -> String {
        "delete_missing_brightness".to_string()
    }

    fn get_cron_schedule(&self) -> String {
        todo!()
    }

    async fn perform_indicator_check_action(&self, pool: &SqlitePool) -> Result<ActionIndicatorCheckMessage> {
        let (difference_total, msg) = get_brightness_missing_on_disk_count(pool).await?;
        Ok(ActionIndicatorCheckMessage(difference_total != 0, msg))
    }
}