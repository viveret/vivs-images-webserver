use std::error::Error;

use async_trait::async_trait;
use convert_case::{Case, Casing};
use nameof::name_of_type;
use sqlx::SqlitePool;

use crate::actions::action_indicator::{ActionIndicatorCheckMessage, IActionIndicator};
use crate::metrics::ocr_text_metrics::{get_ocr_text_missing_in_sql_count, get_ocr_text_missing_on_disk_count};



pub struct ImagesOnDiskWithMissingOcrTextIndicator {}
impl ImagesOnDiskWithMissingOcrTextIndicator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IActionIndicator for ImagesOnDiskWithMissingOcrTextIndicator {
    fn get_name(&self) -> String {
        name_of_type!(ImagesOnDiskWithMissingOcrTextIndicator).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(ImagesOnDiskWithMissingOcrTextIndicator).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the ocr_text table is missing any images that are on the disk".to_string()
    }

    fn get_action_name(&self) -> String {
        "add_ocr_text".to_string()
    }

    fn get_cron_schedule(&self) -> String {
        todo!()
    }

    async fn perform_indicator_check_action(&self, pool: &SqlitePool) -> Result<ActionIndicatorCheckMessage, Box<dyn Error + Send>> {
        let (difference_total, msg) = get_ocr_text_missing_in_sql_count(pool).await?;
        Ok(ActionIndicatorCheckMessage(difference_total != 0, msg))
    }
}



pub struct ImagesInOcrTextSqlDbWithMissingImageOnDiskIndicator {}
impl ImagesInOcrTextSqlDbWithMissingImageOnDiskIndicator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IActionIndicator for ImagesInOcrTextSqlDbWithMissingImageOnDiskIndicator {
    fn get_name(&self) -> String {
        name_of_type!(ImagesInOcrTextSqlDbWithMissingImageOnDiskIndicator).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(ImagesInOcrTextSqlDbWithMissingImageOnDiskIndicator).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the ocr_text table has values for images that are not found or valid on the disk".to_string()
    }

    fn get_action_name(&self) -> String {
        "delete_missing_ocr_text".to_string()
    }

    fn get_cron_schedule(&self) -> String {
        todo!()
    }

    async fn perform_indicator_check_action(&self, pool: &SqlitePool) -> Result<ActionIndicatorCheckMessage, Box<dyn Error + Send>> {
        let (difference_total, msg) = get_ocr_text_missing_on_disk_count(pool).await?;
        Ok(ActionIndicatorCheckMessage(difference_total != 0, msg))
    }
}