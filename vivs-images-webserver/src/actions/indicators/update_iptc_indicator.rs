use std::error::Error;

use async_trait::async_trait;
use convert_case::{Case, Casing};
use nameof::name_of_type;
use sqlx::SqlitePool;

use crate::actions::action_indicator::{ActionIndicatorCheckMessage, IActionIndicator};
use crate::metrics::iptc_metrics::{get_iptc_missing_in_sql_count, get_iptc_missing_on_disk_count};



pub struct ImagesOnDiskWithMissingIptcIndicator {}
impl ImagesOnDiskWithMissingIptcIndicator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IActionIndicator for ImagesOnDiskWithMissingIptcIndicator {
    fn get_name(&self) -> String {
        name_of_type!(ImagesOnDiskWithMissingIptcIndicator).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(ImagesOnDiskWithMissingIptcIndicator).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the Iptc table is missing any images that are on the disk".to_string()
    }

    fn get_action_name(&self) -> String {
        "add_iptc".to_string()
    }

    fn get_cron_schedule(&self) -> String {
        todo!()
    }

    async fn perform_indicator_check_action(&self, pool: &SqlitePool) -> Result<ActionIndicatorCheckMessage, Box<dyn Error + Send>> {
        let (difference_total, msg) = get_iptc_missing_in_sql_count(pool).await?;
        Ok(ActionIndicatorCheckMessage(difference_total != 0, msg))
    }
}



pub struct ImagesInIptcSqlDbWithMissingImageOnDiskIndicator {}
impl ImagesInIptcSqlDbWithMissingImageOnDiskIndicator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IActionIndicator for ImagesInIptcSqlDbWithMissingImageOnDiskIndicator {
    fn get_name(&self) -> String {
        name_of_type!(ImagesInIptcSqlDbWithMissingImageOnDiskIndicator).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(ImagesInIptcSqlDbWithMissingImageOnDiskIndicator).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the Iptc table has values for images that are not found or valid on the disk".to_string()
    }

    fn get_action_name(&self) -> String {
        "delete_missing_iptc".to_string()
    }

    fn get_cron_schedule(&self) -> String {
        todo!()
    }

    async fn perform_indicator_check_action(&self, pool: &SqlitePool) -> Result<ActionIndicatorCheckMessage, Box<dyn Error + Send>> {
        let (difference_total, msg) = get_iptc_missing_on_disk_count(pool).await?;
        Ok(ActionIndicatorCheckMessage(difference_total != 0, msg))
    }
}