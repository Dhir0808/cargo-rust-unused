pub mod analyzer;
pub mod dependencies;
pub mod reporter;
pub mod scanner;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_analyzer_finds_unused_items() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let project_dir = temp_dir.path();

        // Create a test project structure
        fs::create_dir(project_dir.join("src"))?;

        // Create Cargo.toml with real crates
        fs::write(
            project_dir.join("Cargo.toml"),
            r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
log = "0.4"
"#,
        )?;

        // Create src/main.rs with unused items
        fs::write(
            project_dir.join("src").join("main.rs"),
            r#"
use serde;  // This dependency is used

fn used_function() -> &'static str {
    "This function is used"
}

fn unused_function() -> &'static str {
    "This function is never called"
}

mod unused_module {
    pub fn unused_fn() {}
}

fn main() {
    let _ = used_function();  // Explicit function call
}
"#,
        )?;

        let mut analyzer = analyzer::CodeAnalyzer::new();
        analyzer.analyze_project(project_dir)?;

        // Verify unused dependencies
        assert!(
            analyzer
                .report
                .unused_dependencies
                .contains(&"log".to_string()),
            "Expected 'log' to be detected as unused dependency"
        );

        // Verify unused functions
        assert!(
            analyzer
                .report
                .unused_functions
                .contains(&"fn unused_function".to_string()),
            "Expected 'unused_function' to be detected as unused"
        );

        // Verify unused modules
        assert!(
            analyzer
                .report
                .unused_modules
                .contains(&"mod unused_module".to_string()),
            "Expected 'unused_module' to be detected as unused"
        );

        // Verify used items are not reported
        assert!(
            !analyzer
                .report
                .unused_functions
                .contains(&"fn used_function".to_string()),
            "Expected 'used_function' to be detected as used"
        );

        Ok(())
    }
}
