use std::collections::HashSet;

use itertools::Itertools;

use crate::converters::comparison::compare_paths;
use crate::converters::comparison::compare_path_pairs;
use crate::actions::refresh::new_similarity_action::generate_unique_pairs;
use crate::actions::refresh::analysis_task_item_processor::LogProgListenerPair;

#[derive(Clone)]
pub struct FilePathComparisonModel {
    pub total_differences: usize,
    pub files_missing_from_a: HashSet<String>,
    pub files_missing_from_b: HashSet<String>,
    pub message: String,
    pub log: String,
    pub log_error: String,
}

impl FilePathComparisonModel {
    pub fn new(
        paths_a: HashSet<String>, label_a: &str, 
        paths_b: HashSet<String>, label_b: &str,
        log_prog_listener: Option<LogProgListenerPair>
    ) -> Self {
        let mut log = String::new();
        let mut log_error = String::new();
        
        log.push_str(&format!("Comparing {} {} to {} in {}", 
            paths_a.len(), label_a,
            paths_b.len(), label_b
        ));
        
        let (files_missing_from_a, files_missing_from_b, total_differences) = 
            compare_paths(&paths_a, &paths_b, log_prog_listener);

        Self {
            total_differences,
            files_missing_from_a,
            files_missing_from_b,
            message: format!("There are {} file differences", total_differences),
            log, log_error
        }
    }
}

impl std::fmt::Display for FilePathComparisonModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.message)?;
        writeln!(f, "log: {}", self.log)?;
        writeln!(f, "errors: {}", self.log_error)?;
        Ok(())
    }
}

pub struct CrossFilePathComparisonModel {
    pub total_differences: usize,
    pub files_missing_from_a: HashSet<String>,
    pub files_missing_from_b: HashSet<String>,
    pub pairs_missing_from_a: Vec<(String, String)>,
    pub pairs_missing_from_b: Vec<(String, String)>,
    pub message: String,
    pub log: String,
    pub log_error: String,
}

impl CrossFilePathComparisonModel {
    pub fn new(
        paths_a: HashSet<String>, 
        label_a: &str, 
        paths_b: HashSet<String>, 
        label_b: &str,
        cross_paths_a: Vec<(String, String)>,
        cross_paths_b: Vec<(String, String)>,
        log_prog_listener: Option<LogProgListenerPair>
    ) -> Self {
        let mut log = String::new();
        let mut log_error = String::new();
        
        // Log the comparison details
        log.push_str(&format!("Comparing {} {} to {} {}\n", 
            paths_a.len(), label_a,
            paths_b.len(), label_b
        ));
        log.push_str(&format!("Comparing {} {} pairs to {} {} pairs", 
            cross_paths_a.len(), label_a,
            cross_paths_b.len(), label_b
        ));
        
        // Compare regular file paths
        let (files_missing_from_a, files_missing_from_b, file_differences) = 
            compare_paths(&paths_a, &paths_b, log_prog_listener.clone());
        
        // Compare cross file path pairs using a dedicated function
        let (pairs_missing_from_a, pairs_missing_from_b, pair_differences) = 
            compare_path_pairs(&cross_paths_a, &cross_paths_b, log_prog_listener);
        
        let total_differences = file_differences + pair_differences;

        Self {
            total_differences,
            files_missing_from_a,
            files_missing_from_b,
            pairs_missing_from_a,
            pairs_missing_from_b,
            message: format!("There are {} total differences ({} file differences, {} pair differences)", 
                total_differences, file_differences, pair_differences),
            log, 
            log_error
        }
    }

    // get the pairs and extrapolating
    // the set of paths a and b. for paths from disk we can generate unique pairs but
    // for from SQL we need to query the actual pairs so we dont do extra work or create problems.
    pub fn new_easy(
        pairs_a: Vec<(String, String)>, 
        label_a: &str, 
        pairs_b: Vec<(String, String)>, 
        label_b: &str,
        log_prog_listener: Option<LogProgListenerPair>
    ) -> Self {
        let paths_a = pairs_a.iter().map(|x| x.0.clone()).chain(pairs_a.iter().map(|x| x.1.clone())).unique().collect();
        let paths_b = pairs_b.iter().map(|x| x.0.clone()).chain(pairs_b.iter().map(|x| x.1.clone())).unique().collect();
        Self::new(paths_a, label_a, paths_b, label_b, pairs_a, pairs_b, log_prog_listener)
    }

    // get the pairs and extrapolating
    // the set of paths a and b. for paths from disk we can generate unique pairs but
    // for from SQL we need to query the actual pairs so we dont do extra work or create problems.
    pub fn new_easy_2(
        paths_a: HashSet<String>, 
        label_a: &str, 
        pairs_b: Vec<(String, String)>, 
        label_b: &str,
        log_prog_listener: Option<LogProgListenerPair>
    ) -> Self {
        let pairs_a = generate_unique_pairs(&paths_a, log_prog_listener.clone());
        let paths_b = pairs_b.iter().map(|x| x.0.clone()).chain(pairs_b.iter().map(|x| x.1.clone())).unique().collect();
        Self::new(paths_a, label_a, paths_b, label_b, pairs_a, pairs_b, log_prog_listener)
    }
}

impl std::fmt::Display for CrossFilePathComparisonModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.message)?;
        
        if !self.files_missing_from_a.is_empty() {
            writeln!(f, "Files missing from A ({}):", self.files_missing_from_a.len())?;
            for file in self.files_missing_from_a.iter().take(10) {
                writeln!(f, "  - {}", file)?;
            }
            writeln!(f, "...")?;
        }
        
        if !self.files_missing_from_b.is_empty() {
            writeln!(f, "Files missing from B ({}):", self.files_missing_from_b.len())?;
            for file in self.files_missing_from_b.iter().take(10) {
                writeln!(f, "  - {}", file)?;
            }
            writeln!(f, "...")?;
        }
        
        if !self.pairs_missing_from_a.is_empty() {
            writeln!(f, "Pairs missing from A ({}):", self.pairs_missing_from_a.len())?;
            for (first, second) in self.pairs_missing_from_a.iter().take(10) {
                writeln!(f, "  - ({}, {})", first, second)?;
            }
            writeln!(f, "...")?;
        }
        
        if !self.pairs_missing_from_b.is_empty() {
            writeln!(f, "Pairs missing from B ({}):", self.pairs_missing_from_b.len())?;
            for (first, second) in self.pairs_missing_from_b.iter().take(10) {
                writeln!(f, "  - ({}, {})", first, second)?;
            }
            writeln!(f, "...")?;
        }
        
        writeln!(f, "log: {}", self.log)?;
        writeln!(f, "errors: {}", self.log_error)?;
        Ok(())
    }
}

// Clone implementation for CrossFilePathComparisonModel
impl Clone for CrossFilePathComparisonModel {
    fn clone(&self) -> Self {
        Self {
            total_differences: self.total_differences,
            files_missing_from_a: self.files_missing_from_a.clone(),
            files_missing_from_b: self.files_missing_from_b.clone(),
            pairs_missing_from_a: self.pairs_missing_from_a.clone(),
            pairs_missing_from_b: self.pairs_missing_from_b.clone(),
            message: self.message.clone(),
            log: self.log.clone(),
            log_error: self.log_error.clone(),
        }
    }
}