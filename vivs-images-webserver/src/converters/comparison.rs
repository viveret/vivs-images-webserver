
// Compares disk and database paths, returning differences
pub fn compare_paths(disk_paths: &[String], db_paths: &[String]) -> (Vec<String>, Vec<String>, usize) {
    let missing_from_db: Vec<String> = disk_paths.iter()
        .filter(|path| !db_paths.contains(path))
        .cloned()
        .collect();
        
    let missing_from_disk: Vec<String> = db_paths.iter()
        .filter(|path| !disk_paths.contains(path))
        .cloned()
        .collect();
        
    let total_differences = missing_from_db.len() + missing_from_disk.len();
    
    (missing_from_db, missing_from_disk, total_differences)
}