use std::collections::HashSet;

use crate::calc::math::calculate_progress;
use crate::actions::analysis_task_item_processor::LogProgListenerPair;


// Compares disk and database paths, returning differences
pub fn compare_paths(
    paths_a: &HashSet<String>,
    paths_b: &HashSet<String>,
    log_prog_listener: Option<LogProgListenerPair>
) -> (HashSet<String>, HashSet<String>, usize) {
    if let Some(progress_fn) = log_prog_listener {
        progress_fn.0(0.0);
        let total = paths_a.len();
        progress_fn.1(&format!("Comparing {} paths to {} paths (missing from A)", total, paths_b.len()));
        let missing_from_b: HashSet<String> = paths_a.iter()
            .enumerate()
            .filter(|(i, path)| {
                progress_fn.0(calculate_progress(*i, total));
                !paths_b.contains(*path)
            })
            .map(|x| x.1.to_string())
            .collect();
            
        progress_fn.0(0.0);
        let total = paths_b.len();
        progress_fn.1(&format!("Comparing {} paths to {} paths (missing from B)", total, paths_a.len()));
        let missing_from_a: HashSet<String> = paths_b.iter()
            .enumerate()
            .filter(|(i, path)| {
                progress_fn.0(calculate_progress(*i, total));
                !paths_a.contains(*path)
            })
            .map(|x| x.1.to_string())
            .collect();
            
        let total_differences = missing_from_b.len() + missing_from_a.len();
        
        (missing_from_a, missing_from_b, total_differences)
    } else {
        let missing_from_b: HashSet<String> = paths_a.iter()
            .filter(|path| !paths_b.contains(*path))
            .cloned()
            .collect();
            
        let missing_from_a: HashSet<String> = paths_b.iter()
            .filter(|path| !paths_a.contains(*path))
            .cloned()
            .collect();
            
        let total_differences = missing_from_b.len() + missing_from_a.len();
        
        (missing_from_a, missing_from_b, total_differences)
    }
}



// Helper function to compare path pairs
pub fn compare_path_pairs(
    pairs_a: &[(String, String)], 
    pairs_b: &[(String, String)],
    log_prog_listener: Option<LogProgListenerPair>
) -> (Vec<(String, String)>, Vec<(String, String)>, usize) {
    // these might be taking some time, might need to make this the default type for path lists
    let set_a: std::collections::HashSet<_> = pairs_a.iter().collect();
    let set_b: std::collections::HashSet<_> = pairs_b.iter().collect();

    if let Some(progress_fn) = log_prog_listener {
        progress_fn.0(0.0);
        let total = pairs_a.len();
        progress_fn.1(&format!("Comparing {} path pairs to {} paths (missing from A)", total, set_b.len()));
        
        let missing_from_b: Vec<(String, String)> = set_a.iter()
            .enumerate()
            .filter(|(i, path)| {
                progress_fn.0(calculate_progress(*i, total));
                !set_b.contains(*path)
            })
            .map(|pair| *pair.1)
            .cloned()
            .collect();
            
        progress_fn.0(0.0);
        let total = pairs_b.len();
        progress_fn.1(&format!("Comparing {} path pairs to {} paths (missing from B)", total, set_a.len()));

        let missing_from_a: Vec<(String, String)> = set_b.iter()
            .enumerate()
            .filter(|(i, path)| {
                progress_fn.0(calculate_progress(*i, total));
                !set_a.contains(*path)
            })
            .map(|pair| *pair.1)
            .cloned()
            .collect();
            
        let total_differences = missing_from_a.len() + missing_from_b.len();
        
        (missing_from_a, missing_from_b, total_differences)
    } else {
        let missing_from_b: Vec<(String, String)> = set_a
            .difference(&set_b)
            .map(|pair| *pair)
            .cloned()
            .collect();
            
        let missing_from_a: Vec<(String, String)> = set_b
            .difference(&set_a)
            .map(|pair| *pair)
            .cloned()
            .collect();
            
        let total_differences = missing_from_a.len() + missing_from_b.len();
        
        (missing_from_a, missing_from_b, total_differences)
    }
}