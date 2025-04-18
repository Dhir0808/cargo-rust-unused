use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AnalysisReport {
    pub unused_dependencies: Vec<String>,
    pub unused_functions: Vec<String>,
    pub unused_modules: Vec<String>,
}

impl AnalysisReport {
    pub fn new() -> Self {
        Self {
            unused_dependencies: Vec::new(),
            unused_functions: Vec::new(),
            unused_modules: Vec::new(),
        }
    }
}
