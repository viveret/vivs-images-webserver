use std::error::Error;

use async_trait::async_trait;
use convert_case::{Case, Casing};
use nameof::name_of_type;
use sqlx::SqlitePool;

use crate::actions::action_indicator::{ActionIndicatorCheckMessage, IActionIndicator};
use crate::database::query::query_image_similarity::{get_count_of_comparisons_per_image_path, get_count_of_image_paths_from_db};
use crate::metrics::similarity_metrics::{get_simple_similarity_missing_in_sql_count, get_simple_similarity_missing_on_disk_count};



pub struct ImagesOnDiskWithMissingSimilarityIndicator {}
impl ImagesOnDiskWithMissingSimilarityIndicator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IActionIndicator for ImagesOnDiskWithMissingSimilarityIndicator {
    fn get_name(&self) -> String {
        name_of_type!(ImagesOnDiskWithMissingSimilarityIndicator).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(ImagesOnDiskWithMissingSimilarityIndicator).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the similarity table is missing any images that are on the disk".to_string()
    }

    fn get_action_name(&self) -> String {
        "add_from_disk_similarity".to_string()
    }

    fn get_cron_schedule(&self) -> String {
        todo!()
    }

    async fn perform_indicator_check_action(&self, pool: &SqlitePool) -> Result<ActionIndicatorCheckMessage, Box<dyn Error + Send>> {
        let (difference_total, msg) = get_simple_similarity_missing_in_sql_count(pool).await?;
        Ok(ActionIndicatorCheckMessage(difference_total != 0, msg))
    }
}


pub struct ImagesInSqlDbWithLessThanExpectedSimilarityIndicator {}
impl ImagesInSqlDbWithLessThanExpectedSimilarityIndicator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IActionIndicator for ImagesInSqlDbWithLessThanExpectedSimilarityIndicator {
    fn get_name(&self) -> String {
        name_of_type!(ImagesInSqlDbWithLessThanExpectedSimilarityIndicator).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(ImagesInSqlDbWithLessThanExpectedSimilarityIndicator).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the similarity table is missing any entries that are in the sql db of known images".to_string()
    }

    fn get_action_name(&self) -> String {
        "add_from_db_similarity".to_string()
    }

    fn get_cron_schedule(&self) -> String {
        todo!()
    }

    async fn perform_indicator_check_action(&self, pool: &SqlitePool) -> Result<ActionIndicatorCheckMessage, Box<dyn Error + Send>> {
        let all = get_count_of_comparisons_per_image_path(pool).await?;
        let expected_total_for_each = get_count_of_image_paths_from_db(pool).await?;
        let difference_total = all.iter().filter(|x| *x.1 != expected_total_for_each).count();
        let msg= format!("there are {} images without the expected amount of {} entries", difference_total, expected_total_for_each);
        Ok(ActionIndicatorCheckMessage(difference_total != 0, msg))
    }
}



pub struct ImagesInSimilaritySqlDbWithMissingImageOnDiskIndicator {}
impl ImagesInSimilaritySqlDbWithMissingImageOnDiskIndicator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl IActionIndicator for ImagesInSimilaritySqlDbWithMissingImageOnDiskIndicator {
    fn get_name(&self) -> String {
        name_of_type!(ImagesInSimilaritySqlDbWithMissingImageOnDiskIndicator).to_case(Case::Snake)
    }

    fn get_label(&self) -> String {
        name_of_type!(ImagesInSimilaritySqlDbWithMissingImageOnDiskIndicator).to_case(Case::Sentence)
    }

    fn get_description(&self) -> String {
        "If the similarity table has values for images that are not found or valid on the disk".to_string()
    }

    fn get_action_name(&self) -> String {
        "delete_missing_similarity".to_string()
    }

    fn get_cron_schedule(&self) -> String {
        todo!()
    }

    async fn perform_indicator_check_action(&self, pool: &SqlitePool) -> Result<ActionIndicatorCheckMessage, Box<dyn Error + Send>> {
        let (difference_total, msg) = get_simple_similarity_missing_on_disk_count(pool).await?;
        Ok(ActionIndicatorCheckMessage(difference_total != 0, msg))
    }
}