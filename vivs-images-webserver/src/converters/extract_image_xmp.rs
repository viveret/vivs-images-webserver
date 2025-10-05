use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

use crate::models::image_xmp::ImageXmp;

pub fn extract_image_xmp(image_path: &str) -> Result<Option<String>, Box<dyn Error + Send>> {
    let metadata = rexiv2::Metadata::new_from_path(Path::new(image_path))
        .map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;
    
    // Get XMP data as string
    if let Ok(xmp_keys) = metadata.get_xmp_tags() {
        if !xmp_keys.is_empty() {
            let mut xmp_data = HashMap::new();
            for k in xmp_keys {
                match metadata.get_tag_string(&k) {
                    Ok(v) => {
                        xmp_data.insert(k, v);
                    }
                    Err(e) => {
                        panic!("error getting xmp: {}", e);
                    }
                }
            }
            let json = serde_json::json!(xmp_data);
            return Ok(Some(json.to_string()));
        }
    }
    
    Ok(None)
}

pub fn extract_image_xmp_model(image_path: &str) -> Result<Option<ImageXmp>, Box<dyn Error + Send>> {
    let xmp = extract_image_xmp(image_path)?
        .map(|xmp| ImageXmp {
        xmp,
        image_path: image_path.to_string(),
    });
    Ok(xmp)
}