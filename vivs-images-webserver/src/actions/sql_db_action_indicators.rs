use std::rc::Rc;

use crate::actions::indicators::update_aspect_ratio_indicator::ImagesInAspectRatioSqlDbWithMissingImageOnDiskIndicator;
use crate::actions::indicators::update_aspect_ratio_indicator::ImagesOnDiskWithMissingAspectRatioIndicator;
use crate::actions::indicators::update_brightness_indicator::ImagesOnDiskWithMissingBrightnessIndicator;
use crate::actions::indicators::update_brightness_indicator::ImagesInBrightnessSqlDbWithMissingImageOnDiskIndicator;
use crate::actions::action_indicator::IActionIndicator;
use crate::actions::indicators::update_exif_indicator::ImagesOnDiskWithMissingExifIndicator;
use crate::actions::indicators::update_exif_indicator::ImagesInExifSqlDbWithMissingImageOnDiskIndicator;
use crate::actions::indicators::update_image_paths_indicator::ImagesOnDiskWithMissingImagePathsIndicator;
use crate::actions::indicators::update_iptc_indicator::ImagesInIptcSqlDbWithMissingImageOnDiskIndicator;
use crate::actions::indicators::update_iptc_indicator::ImagesOnDiskWithMissingIptcIndicator;
use crate::actions::indicators::update_ocr_text_indicator::ImagesInOcrTextSqlDbWithMissingImageOnDiskIndicator;
use crate::actions::indicators::update_ocr_text_indicator::ImagesOnDiskWithMissingOcrTextIndicator;
use crate::actions::indicators::update_similarity_indicator::ImagesInSimilaritySqlDbWithMissingImageOnDiskIndicator;
use crate::actions::indicators::update_similarity_indicator::ImagesInSqlDbWithLessThanExpectedSimilarityIndicator;
use crate::actions::indicators::update_similarity_indicator::ImagesOnDiskWithMissingSimilarityIndicator;
use crate::actions::indicators::update_tags_indicator::ImagesOnDiskWithMissingTagsIndicator;
use crate::actions::indicators::update_thumbnail_indicator::ImagesInThumbnailSqlDbWithMissingImageOnDiskIndicator;
use crate::actions::indicators::update_thumbnail_indicator::ImagesOnDiskWithMissingThumbnailIndicator;
use crate::actions::indicators::update_xmp_indicator::ImagesInXmpSqlDbWithMissingImageOnDiskIndicator;
use crate::actions::indicators::update_xmp_indicator::ImagesOnDiskWithMissingXmpIndicator;



pub fn get_sql_db_action_indicators() -> Vec<Rc<dyn IActionIndicator>> {
    vec![
        Rc::new(ImagesOnDiskWithMissingImagePathsIndicator::new()),
        Rc::new(ImagesInBrightnessSqlDbWithMissingImageOnDiskIndicator::new()),
        Rc::new(ImagesOnDiskWithMissingBrightnessIndicator::new()),
        Rc::new(ImagesInExifSqlDbWithMissingImageOnDiskIndicator::new()),
        Rc::new(ImagesOnDiskWithMissingExifIndicator::new()),
        Rc::new(ImagesInSimilaritySqlDbWithMissingImageOnDiskIndicator::new()),
        Rc::new(ImagesInSqlDbWithLessThanExpectedSimilarityIndicator::new()),
        Rc::new(ImagesOnDiskWithMissingSimilarityIndicator::new()),
        Rc::new(ImagesInThumbnailSqlDbWithMissingImageOnDiskIndicator::new()),
        Rc::new(ImagesOnDiskWithMissingThumbnailIndicator::new()),
        Rc::new(ImagesOnDiskWithMissingOcrTextIndicator::new()),
        Rc::new(ImagesInOcrTextSqlDbWithMissingImageOnDiskIndicator::new()),
        Rc::new(ImagesInAspectRatioSqlDbWithMissingImageOnDiskIndicator::new()),
        Rc::new(ImagesOnDiskWithMissingAspectRatioIndicator::new()),
        Rc::new(ImagesOnDiskWithMissingIptcIndicator::new()),
        Rc::new(ImagesInIptcSqlDbWithMissingImageOnDiskIndicator::new()),
        Rc::new(ImagesOnDiskWithMissingTagsIndicator::new()),
        Rc::new(ImagesOnDiskWithMissingXmpIndicator::new()),
        Rc::new(ImagesInXmpSqlDbWithMissingImageOnDiskIndicator::new()),
    ]
}