use anyhow::{Context, Result};
use cargo_metadata::{MetadataCommand, Package};
use std::path::Path;

pub fn get_project_dependencies<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let metadata = MetadataCommand::new()
        .manifest_path(path.as_ref().join("Cargo.toml"))
        .exec()
        .with_context(|| "Failed to get cargo metadata")?;

    let root_package = metadata
        .root_package()
        .with_context(|| "Failed to find root package")?;

    let mut dependencies = Vec::new();
    collect_dependencies(root_package, &mut dependencies);

    Ok(dependencies)
}

fn collect_dependencies(package: &Package, deps: &mut Vec<String>) {
    for dep in &package.dependencies {
        deps.push(dep.name.clone());
    }
}
