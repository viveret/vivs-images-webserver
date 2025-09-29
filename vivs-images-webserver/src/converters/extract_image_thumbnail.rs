use std::path::Path;
use std::io::{Error, ErrorKind, Result};

use image::codecs::bmp::BmpEncoder;
use image::codecs::jpeg::JpegEncoder;
use image::{DynamicImage, ImageEncoder};
use image::{codecs::png::PngEncoder, imageops::FilterType};

use crate::models::image_thumbnail::{ImageThumbnail, ThumbnailFormat};



pub struct ExtractImageThumbnailOptions {
    pub width_and_length: u32,
    pub filter: FilterType,
    pub output_format: ThumbnailFormat,
}

pub fn convert_image_to_thumbnail_image(img: &DynamicImage, options: &ExtractImageThumbnailOptions) -> Result<DynamicImage> {
    Ok(img.resize(options.width_and_length, options.width_and_length, options.filter))
}

pub fn convert_image_to_vec_u8(img: &DynamicImage, options: &ExtractImageThumbnailOptions) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    
    match options.output_format {
        ThumbnailFormat::PNG => {
            let encoder = PngEncoder::new(&mut buf);
            encoder.write_image(
                img.as_bytes(),
                img.width(),
                img.height(),
                img.color().into(),
            ).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        },
        ThumbnailFormat::JPG => {
            let encoder = JpegEncoder::new(&mut buf);
            encoder.write_image(
                img.as_bytes(),
                img.width(),
                img.height(),
                img.color().into(),
            ).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        },
        ThumbnailFormat::BMP => {
            let encoder = BmpEncoder::new(&mut buf);
            encoder.write_image(
                img.as_bytes(),
                img.width(),
                img.height(),
                img.color().into(),
            ).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        },
        ThumbnailFormat::RAW_rgb8 => {
            // For raw format, just return the raw bytes
            buf = img.to_rgb8().into_raw();
        },
    }

    Ok(buf)
}

pub fn convert_image_to_thumbnail(img: &DynamicImage, options: &ExtractImageThumbnailOptions) -> Result<Vec<u8>> {
    let new_image = convert_image_to_thumbnail_image(img, options)?;
    convert_image_to_vec_u8(&new_image, options)
}

pub async fn extract_image_thumbnail(path: &str, options: ExtractImageThumbnailOptions) -> Result<ImageThumbnail> {
    let img = image::open(Path::new(path))
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    let buf = convert_image_to_thumbnail(&img, &options)?;
    
    Ok(ImageThumbnail::new(
        path.to_string(), 
        options.width_and_length, 
        options.output_format,
        buf
    ))
}

pub fn extract_multiple_image_thumbnails(size_list: &[u32], img: &DynamicImage, filter: FilterType) -> Result<Vec<DynamicImage>> {
    if size_list.is_empty() {
        return Ok(Vec::new());
    }

    let mut sorted_sizes: Vec<u32> = size_list.to_vec();
    sorted_sizes.sort_by(|a, b| b.cmp(a));
    
    let mut thumbnails = Vec::with_capacity(sorted_sizes.len());
    
    for (i, &target_size) in sorted_sizes.iter().enumerate() {
        let source_image = if i == 0 {
            img
        } else {
            &thumbnails[i - 1]
        };

        thumbnails.push(
            source_image.resize(target_size, target_size, filter)
        );
    }

    Ok(thumbnails)
}

pub fn open_and_extract_multiple_image_thumbnails_standard_sizes(path: &str) -> Result<Vec<DynamicImage>> {
    let img = image::open(Path::new(path))
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    extract_multiple_image_thumbnails_standard_sizes(&img)
}

pub const DEFAULT_THUMBNAIL_SIZE_LIST: [u32;5] = [8, 16, 32, 64, 128];

pub fn extract_multiple_image_thumbnails_standard_sizes(img: &DynamicImage) -> Result<Vec<DynamicImage>> {
    let filter = image::imageops::FilterType::Lanczos3;
    extract_multiple_image_thumbnails(&DEFAULT_THUMBNAIL_SIZE_LIST, img, filter)
}

pub fn extract_multiple_image_thumbnails_standard_sizes_to_png_vec_u8(img: &DynamicImage) -> Result<Vec<Vec<u8>>> {
    let thumbs= extract_multiple_image_thumbnails_standard_sizes(img)?;
    convert_thumbnails_to_vec_vec_u8(thumbs)
}

pub fn convert_thumbnails_to_vec_vec_u8(thumbs: Vec<DynamicImage>) -> std::result::Result<Vec<Vec<u8>>, Error> {
    let mut imgs = vec![];
    for img in thumbs {
        let options = ExtractImageThumbnailOptions {
            filter: image::imageops::FilterType::Lanczos3,
            width_and_length: img.width(),
            output_format: ThumbnailFormat::PNG
        };
        let s = convert_image_to_vec_u8(&img, &options)?;
        imgs.push(s);
    }
    Ok(imgs)
}



pub fn convert_vec_u8_to_image(img: &Vec<u8>) -> Result<DynamicImage> {
    image::load_from_memory(img)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}