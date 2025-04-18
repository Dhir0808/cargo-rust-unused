use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn scan_rust_files<P: AsRef<Path>>(path: P) -> Result<Vec<PathBuf>> {
    let mut rust_files = Vec::new();
    let path = path.as_ref();

    // First, check if this is a Cargo project by looking for Cargo.toml
    let cargo_toml = path.join("Cargo.toml");
    if !cargo_toml.exists() {
        return Err(anyhow::anyhow!("Not a Cargo project: no Cargo.toml found"));
    }

    // Look for Rust files in the src directory
    let src_dir = path.join("src");
    if !src_dir.exists() {
        return Err(anyhow::anyhow!("No src directory found"));
    }

    for entry in WalkDir::new(&src_dir)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| !is_hidden(e.path()))
    {
        let entry = entry.with_context(|| "Failed to read directory entry")?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            // Skip generated files and tests
            if !path.to_string_lossy().contains("/target/") && !is_test_file(path) {
                rust_files.push(path.to_owned());
            }
        }
    }

    Ok(rust_files)
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|s| s.to_str())
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
        || path.to_string_lossy().contains("/target/")
}

fn is_test_file(path: &Path) -> bool {
    path.to_string_lossy().contains("tests/") || path.to_string_lossy().ends_with("_test.rs")
}
