use std::path::PathBuf;
use std::error::Error;
use std::collections::HashSet;

use crate::models::image_iptc::ImageIptc;


pub fn extract_image_iptc(image_path: &str) -> Result<ImageIptc, Box<dyn Error + Send>> {
    let iptc = iptc::IPTC::read_from_path(&PathBuf::from(image_path))
        .map_err(|e| Box::new(std::io::Error::other(e.to_string())) as Box<dyn Error + Send>)?;
    let mut image_iptc = ImageIptc::default(image_path);
    for kvp in iptc.get_all() {
        image_iptc.set_field_value(kvp.0.to_string(), kvp.1.join("\n"))
            .map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;
    }
    Ok(image_iptc)
}

pub fn extract_image_iptc_tags(image_path: &str) -> Result<HashSet<String>, Box<dyn Error + Send>> {
    let mut found_tags = HashSet::new();
    if let Ok(iptc) = iptc::IPTC::read_from_path(&PathBuf::from(image_path)) {
        let tags = iptc.get(iptc::IPTCTag::Keywords);
        if !tags.is_empty() {
            // println!("iptc tags: {}", tags);
            for tag in tags.split(',') {
                found_tags.insert(tag.to_string());
            }
        }
    }
    Ok(found_tags)
}