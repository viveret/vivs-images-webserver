use std::collections::HashSet;



pub struct ImagePaths(pub HashSet<String>);

impl std::fmt::Display for ImagePaths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}