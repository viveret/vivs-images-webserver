use crate::database::create::create_image_aspect_ratio::SQL_CREATE_IMAGE_ASPECT_RATIO;
use crate::database::create::create_image_iptc::SQL_CREATE_IMAGE_IPTC;
use crate::database::create::create_image_ocr_text::SQL_CREATE_IMAGE_OCR_TEXT;
use crate::database::create::create_image_similarity::SQL_CREATE_IMAGE_SIMILARITY;
use crate::database::create::create_image_exif::SQL_CREATE_IMAGE_EXIF;
use crate::database::create::create_image_brightness::SQL_CREATE_IMAGE_BRIGHTNESS;
use crate::database::create::create_image_thumbnail::SQL_CREATE_IMAGE_THUMBNAIL;


pub const SQL_CREATE_IMAGE_TABLES: &[&str] = &[
    SQL_CREATE_IMAGE_BRIGHTNESS,
    SQL_CREATE_IMAGE_EXIF,
    SQL_CREATE_IMAGE_SIMILARITY,
    SQL_CREATE_IMAGE_ASPECT_RATIO,
    SQL_CREATE_IMAGE_OCR_TEXT,
    SQL_CREATE_IMAGE_THUMBNAIL,
    SQL_CREATE_IMAGE_IPTC
];
