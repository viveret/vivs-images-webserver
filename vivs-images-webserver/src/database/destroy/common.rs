use crate::database::destroy::destroy_image_similarity::SQL_DESTROY_IMAGE_SIMILARITY;
use crate::database::destroy::destroy_image_exif::SQL_DESTROY_IMAGE_EXIF;
use crate::database::destroy::destroy_image_brightness::SQL_DESTROY_IMAGE_BRIGHTNESS;



pub const SQL_DESTROY_IMAGE_TABLES: &[&str] = &[
    SQL_DESTROY_IMAGE_BRIGHTNESS,
    SQL_DESTROY_IMAGE_EXIF,
    SQL_DESTROY_IMAGE_SIMILARITY,
];