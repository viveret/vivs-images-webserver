use std::rc::Rc;

use crate::actions::indicators::update_brightness_indicator::ImagesOnDiskWithMissingBrightnessIndicator;
use crate::actions::indicators::update_brightness_indicator::ImagesInBrightnessSqlDbWithMissingImageOnDiskIndicator;
use crate::actions::action_indicator::IActionIndicator;
use crate::actions::indicators::update_exif_indicator::ImagesOnDiskWithMissingExifIndicator;
use crate::actions::indicators::update_exif_indicator::ImagesInExifSqlDbWithMissingImageOnDiskIndicator;
use crate::actions::indicators::update_similarity_indicator::ImagesInSimilaritySqlDbWithMissingImageOnDiskIndicator;
use crate::actions::indicators::update_similarity_indicator::ImagesInSqlDbWithLessThanExpectedSimilarityIndicator;
use crate::actions::indicators::update_similarity_indicator::ImagesOnDiskWithMissingSimilarityIndicator;
use crate::actions::indicators::update_thumbnail_indicator::ImagesInThumbnailSqlDbWithMissingImageOnDiskIndicator;
use crate::actions::indicators::update_thumbnail_indicator::ImagesOnDiskWithMissingThumbnailIndicator;



pub fn get_sql_db_action_indicators() -> Vec<Rc<dyn IActionIndicator>> {
    vec![
        Rc::new(ImagesInBrightnessSqlDbWithMissingImageOnDiskIndicator::new()),
        Rc::new(ImagesOnDiskWithMissingBrightnessIndicator::new()),
        Rc::new(ImagesInExifSqlDbWithMissingImageOnDiskIndicator::new()),
        Rc::new(ImagesOnDiskWithMissingExifIndicator::new()),
        Rc::new(ImagesInSimilaritySqlDbWithMissingImageOnDiskIndicator::new()),
        Rc::new(ImagesInSqlDbWithLessThanExpectedSimilarityIndicator::new()),
        Rc::new(ImagesOnDiskWithMissingSimilarityIndicator::new()),
        Rc::new(ImagesInThumbnailSqlDbWithMissingImageOnDiskIndicator::new()),
        Rc::new(ImagesOnDiskWithMissingThumbnailIndicator::new()),
    ]
}