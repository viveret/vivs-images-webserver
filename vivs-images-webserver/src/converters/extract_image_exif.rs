use std::collections::HashSet;

use exif::{In, Tag};

use crate::models::image_exif::ImageExif;
use crate::converters::extract_image_taken_at::extract_image_taken_at;


pub fn extract_image_exif(image_path: &str) -> actix_web::Result<ImageExif> {
    let file = std::fs::File::open(image_path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let mut image_exif = ImageExif::default(image_path);
    for enum2 in exif.fields() {
        if let exif::Value::Ascii(strings) = &enum2.value {
            // the strings need to be checked for length before setting. otherwise we will get a lot of empty strings
            let mut already_set_value = false;
            for s in strings {
                match String::from_utf8(s.clone()) {
                    Ok(s) => {
                        // todo: this only works for new entries, not updating because we sometimes want empty
                        // if the value was actually cleared in the file on disk.
                        if !s.is_empty() {
                            if already_set_value {
                                panic!("already set value for {}", &enum2.tag);
                            }
                            image_exif.set_field_by_tag(&enum2.tag, s.as_str());
                            // println!("extract utf8-string from exif data in {}: {} = {}", &enum2.tag, image_path, s);
                            already_set_value = true;
                        }
                    },
                    Err(e) => {
                        println!("extract utf8-string from exif data in {} error: {}", image_path, e);
                    }
                }
            }
        } else {
            image_exif.set_field_by_tag(&enum2.tag, enum2.display_value().to_string().as_str());
        }
    }
    
    image_exif.image_taken_at = extract_image_taken_at(image_path)?;

    Ok(image_exif)
}

pub fn extract_image_exif_tags(image_path: &str) -> std::io::Result<HashSet<String>> {
    let file = std::fs::File::open(image_path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();

    let mut found_tags = HashSet::new();
    if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
        let description: String = exif.get_field(Tag::ImageDescription, In::PRIMARY).map(|v| v.display_value().to_string()).unwrap_or_default();
        for tags_indicator in ["tags:", "keywords:"] {
            if let Some(tags_start_index) = description.find(tags_indicator) {
                let tags = &description[tags_start_index..];
                println!("exif description tags: {}", tags);
                for tag in tags.split(',') {
                    found_tags.insert(tag.to_string());
                }
            }
        }
    }

    Ok(found_tags)
}