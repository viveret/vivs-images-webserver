use std::collections::HashSet;
use std::path::Path;
use homedir::my_home;

// Common image extensions for reuse
pub const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "bmp", "gif", "webp"];
pub const TEXT_EXTENSIONS: &[&str] = &["txt"];

// Helper function to check if a file has any of the given extensions
pub fn has_extension(file_name: &str, extensions: &[&str]) -> bool {
    let file_name_lower = file_name.to_lowercase();
    extensions.iter().any(|&ext| file_name_lower.ends_with(&format!(".{}", ext)))
}

// Generic function to get files with specific extensions in a folder
pub fn get_files_in_folder<P: AsRef<Path>>(folder: P, extensions: &[&str]) -> HashSet<String> {
    let mut results = HashSet::new();
    let folder_path = folder.as_ref();

    if let Ok(entries) = std::fs::read_dir(folder_path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if has_extension(file_name, extensions) {
                            if let Ok(full_path) = entry.path().canonicalize() {
                                results.insert(full_path.to_string_lossy().into_owned());
                            }
                        }
                    }
                } else if file_type.is_dir() {
                    let subfolder_files = get_files_in_folder(entry.path(), extensions);
                    results.extend(subfolder_files);
                } else {
                    println!("unexpected file type {:?}", file_type)
                }
            }
        }
    }
    
    results
}

fn get_sync_path(base_dir: &str, sync_folder: &str, subfolder: Option<String>) -> actix_web::Result<String> {
    let mut sync_path = my_home()
        .ok()
        .flatten()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Home directory not found"))?;
    
    sync_path.push(format!("{}/{}", base_dir, sync_folder));
    if let Some(subfolder) = subfolder {
        sync_path.push(subfolder);
    }
    
    let canonical_path = sync_path.canonicalize()
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Path canonicalization failed: {}", e)))?;
    
    canonical_path.to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Path contains invalid UTF-8"))
}

pub fn get_images_in_folder(folder: String) -> HashSet<String> {
    get_files_in_folder(folder, IMAGE_EXTENSIONS)
}

pub fn get_photo_sync_path() -> actix_web::Result<String> {
    get_sync_path("Pictures", "photo-sync.git", None)
}

pub fn get_images_in_photo_sync_path() -> actix_web::Result<HashSet<String>> {
    let images_path = get_photo_sync_path()?;
    Ok(get_images_in_folder(images_path))
}

pub fn get_ocr_text_file_paths_in_folder(folder: String) -> HashSet<String> {
    get_files_in_folder(folder, TEXT_EXTENSIONS)
}

pub fn get_doc_sync_path(subfolder: Option<String>) -> actix_web::Result<String> {
    get_sync_path("Documents", "doc-sync.git", subfolder)
}

pub fn get_image_ocr_text_export_path() -> actix_web::Result<String> {
    get_doc_sync_path(Some("image_ocr_text_export/".to_string()))
}

pub fn get_ocr_text_file_paths_in_doc_sync_path() -> actix_web::Result<HashSet<String>> {
    let docs_path = get_image_ocr_text_export_path()?;
    Ok(get_ocr_text_file_paths_in_folder(docs_path))
}



pub fn change_base_path_of_paths(files: HashSet<String>, old_base: String, new_base: String) -> std::io::Result<HashSet<String>> {
    let ocr_text_file_paths: HashSet<String> = files.iter().filter_map(|f| {
        change_base_path(f, &old_base, &new_base)
    }).collect();
    Ok(ocr_text_file_paths)
}

pub fn change_base_path(f: &str, old_base: &str, new_base: &str) -> Option<String> {
    if f.starts_with(&old_base) {
        Some(format!("{}{}", new_base, &f[old_base.len()..]))
    } else {
        None
    }
}