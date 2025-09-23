use homedir::my_home;


pub fn get_images_in_folder(folder: String) -> Vec<String> {
    let mut results = Vec::new();
    // println!("traversing folder {}", folder);

    if let Ok(entries) = std::fs::read_dir(folder) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Some(file_name) = entry.file_name().to_str() {
                        let file_name_lower = file_name.to_lowercase();
                        if file_name_lower.ends_with(".jpg") ||
                        file_name_lower.ends_with(".jpeg") ||
                        file_name_lower.ends_with(".png") ||
                        file_name_lower.ends_with(".bmp") ||
                        file_name_lower.ends_with(".gif") ||
                        file_name_lower.ends_with(".webp") {
                            if let Ok(full_path) = entry.path().canonicalize() {
                                results.push(full_path.to_string_lossy().into_owned());
                            }
                        }
                    }
                } else if file_type.is_dir() {
                    results.extend_from_slice(&get_images_in_folder(entry.path().to_str().unwrap().to_string()));
                } else {
                    println!("unexpected file type {:?}", file_type)
                }
            }
        }
    }
    
    results
}

// Gets the canonical path to the photo-sync directory
pub fn get_photo_sync_path() -> actix_web::Result<String> {
    let mut images_path = my_home()
        .ok()
        .flatten()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Home directory not found"))?;
    
    images_path.push("Pictures/photo-sync.git");
    let canonical_path = images_path.canonicalize()
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Path canonicalization failed: {}", e)))?;
    
    canonical_path.to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Path contains invalid UTF-8"))
}

pub fn get_images_in_photo_sync_path() -> actix_web::Result<Vec<String>> {
    let images_path = get_photo_sync_path()?;
    Ok(get_images_in_folder(images_path))
}