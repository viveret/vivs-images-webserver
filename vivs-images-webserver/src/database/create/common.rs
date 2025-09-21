use crate::database::create::create_image_similarity::SQL_CREATE_IMAGE_SIMILARITY;
use crate::database::create::create_image_exif::SQL_CREATE_IMAGE_EXIF;
use crate::database::create::create_image_brightness::SQL_CREATE_IMAGE_BRIGHTNESS;


pub const SQL_CREATE_IMAGE_TABLES: &[&str] = &[
    SQL_CREATE_IMAGE_BRIGHTNESS,
    SQL_CREATE_IMAGE_EXIF,
    SQL_CREATE_IMAGE_SIMILARITY,
];
