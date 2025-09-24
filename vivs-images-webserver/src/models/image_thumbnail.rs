
pub enum ThumbnailFormat { 
    PNG,
    JPG,
    BMP,
    RAW_rgb8,
}

pub struct ImageThumbnail {
    pub image_path: String,
    pub width_and_length: u32,
    pub thumbnail_format: ThumbnailFormat,
    pub thumbnail_data: Vec<u8>,
}

impl ImageThumbnail {
    pub fn new(
        image_path: String,
        width_and_length: u32,
        thumbnail_format: ThumbnailFormat,
        thumbnail_data: Vec<u8>,
    ) -> Self {
        Self {
            image_path,
            width_and_length,
            thumbnail_format,
            thumbnail_data
        }
    }
}