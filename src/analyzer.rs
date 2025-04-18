use anyhow::{Context, Result};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use syn::{visit::Visit, Expr, ExprCall, ExprPath, Item, UseTree};

use crate::dependencies::get_project_dependencies;
use crate::reporter::AnalysisReport;
use crate::scanner::scan_rust_files;

pub struct CodeAnalyzer {
    pub report: AnalysisReport,
    declared_items: HashSet<String>,
    used_items: HashSet<String>,
    dependencies: HashSet<String>,
    used_dependencies: HashSet<String>,
}

impl CodeAnalyzer {
    pub fn new() -> Self {
        Self {
            report: AnalysisReport::new(),
            declared_items: HashSet::new(),
            used_items: HashSet::new(),
            dependencies: HashSet::new(),
            used_dependencies: HashSet::new(),
        }
    }

    pub fn analyze_project<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        // Get project dependencies
        let deps = get_project_dependencies(&path)?;
        self.dependencies.extend(deps);

        // Scan all Rust files
        let rust_files = scan_rust_files(&path)?;

        // First pass: collect all declared items
        for file in &rust_files {
            self.analyze_file(file)?;
        }

        // Find unused items
        self.find_unused_items();

        Ok(())
    }

    fn analyze_file(&mut self, path: &PathBuf) -> Result<()> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        let syntax = syn::parse_file(&content)
            .with_context(|| format!("Failed to parse file: {}", path.display()))?;

        // Visit the AST
        let mut visitor = ItemVisitor {
            declared_items: &mut self.declared_items,
            used_items: &mut self.used_items,
            used_dependencies: &mut self.used_dependencies,
        };
        visitor.visit_file(&syntax);

        Ok(())
    }

    fn find_unused_items(&mut self) {
        // Find unused dependencies
        for dep in &self.dependencies {
            if !self.used_dependencies.contains(dep) {
                self.report.unused_dependencies.push(dep.clone());
            }
        }

        // Find unused items (functions, structs, etc.)
        for item in &self.declared_items {
            if !self.used_items.contains(item) {
                if item.starts_with("fn ") {
                    self.report.unused_functions.push(item.clone());
                } else if item.starts_with("mod ") {
                    self.report.unused_modules.push(item.clone());
                }
            }
        }

        // Sort for consistent output
        self.report.unused_dependencies.sort();
        self.report.unused_functions.sort();
        self.report.unused_modules.sort();
    }
}

struct ItemVisitor<'a> {
    declared_items: &'a mut HashSet<String>,
    used_items: &'a mut HashSet<String>,
    used_dependencies: &'a mut HashSet<String>,
}

impl<'a> Visit<'_> for ItemVisitor<'a> {
    fn visit_item(&mut self, item: &Item) {
        match item {
            Item::Fn(f) => {
                // Skip main function as it's the entry point
                if f.sig.ident != "main" {
                    let fn_name = format!("fn {}", f.sig.ident);
                    self.declared_items.insert(fn_name);
                }
            }
            Item::Mod(m) => {
                let mod_name = format!("mod {}", m.ident);
                self.declared_items.insert(mod_name);
            }
            _ => {}
        }
        syn::visit::visit_item(self, item);
    }

    fn visit_expr(&mut self, expr: &Expr) {
        match expr {
            // Only mark functions as used when they are actually called
            Expr::Call(ExprCall { func, .. }) => {
                if let Expr::Path(ExprPath { path, .. }) = &**func {
                    if let Some(ident) = path.segments.last() {
                        let fn_name = format!("fn {}", ident.ident);
                        self.used_items.insert(fn_name);
                    }
                }
            }
            _ => {}
        }
        syn::visit::visit_expr(self, expr);
    }

    fn visit_use_tree(&mut self, tree: &UseTree) {
        if let UseTree::Path(path) = tree {
            if let Some(ident) = path.ident.to_string().into() {
                self.used_dependencies.insert(ident);
            }
        }
        syn::visit::visit_use_tree(self, tree);
    }
}
