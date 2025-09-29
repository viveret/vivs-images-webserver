use crate::converters::comparison::compare_paths;


#[derive(Clone)]
pub struct FilePathComparisonModel {
    pub total_differences: usize,
    pub files_missing_from_a: Vec<String>,
    pub files_missing_from_b: Vec<String>,
    pub message: String,
    pub log: String,
    pub log_error: String,
}

impl FilePathComparisonModel {
    pub fn new(paths_a: Vec<String>, label_a: &str, paths_b: Vec<String>, label_b: &str) -> Self {
        let mut log = String::new();
        let mut log_error = String::new();
        
        log.push_str(&format!("Comparing {} {} to {} in {}", 
            paths_a.len(), label_a,
            paths_b.len(), label_b
        ));
        
        let (files_missing_from_a, files_missing_from_b, total_differences) = 
            compare_paths(&paths_a, &paths_b);

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