use std::error::Error;

use async_trait::async_trait;
use convert_case::{Case, Casing};
use nameof::name_of_type;
use sqlx::SqlitePool;

use crate::actions::action_indicator::{ActionIndicatorCheckMessage, IActionIndicator};
use crate::metrics::xmp_metrics::{get_xmp_missing_in_sql_count, get_xmp_missing_on_disk_count};



pub struct ImagesOnDiskWithMissingXmpIndicator;
impl ImagesOnDiskWithMissingXmpIndicator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IActionIndicator for ImagesOnDiskWithMissingXmpIndicator {
    fn get_name(&self) -> String {
        name_of_type!(ImagesOnDiskWithMissingXmpIndicator).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(ImagesOnDiskWithMissingXmpIndicator).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the xmp table is missing any images that are on the disk".to_string()
    }

    fn get_action_name(&self) -> String { "add_xmp".to_string() }

    fn get_cron_schedule(&self) -> String { todo!() }

    async fn perform_indicator_check_action(&self, pool: &SqlitePool) -> Result<ActionIndicatorCheckMessage, Box<dyn Error + Send>> {
        let (difference_total, msg) = get_xmp_missing_in_sql_count(pool).await?;
        Ok(ActionIndicatorCheckMessage(difference_total != 0, msg))
    }
}



pub struct ImagesInXmpSqlDbWithMissingImageOnDiskIndicator;
impl ImagesInXmpSqlDbWithMissingImageOnDiskIndicator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IActionIndicator for ImagesInXmpSqlDbWithMissingImageOnDiskIndicator {
    fn get_name(&self) -> String {
        name_of_type!(ImagesInXmpSqlDbWithMissingImageOnDiskIndicator).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(ImagesInXmpSqlDbWithMissingImageOnDiskIndicator).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the xmp table has values for images that are not found or valid on the disk".to_string()
    }

    fn get_action_name(&self) -> String { "delete_missing_xmp".to_string() }

    fn get_cron_schedule(&self) -> String { todo!() }

    async fn perform_indicator_check_action(&self, pool: &SqlitePool) -> Result<ActionIndicatorCheckMessage, Box<dyn Error + Send>> {
        let (difference_total, msg) = get_xmp_missing_on_disk_count(pool).await?;
        Ok(ActionIndicatorCheckMessage(difference_total != 0, msg))
    }
}