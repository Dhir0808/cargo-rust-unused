use anyhow::Result;
use clap::{Parser, ValueEnum};
use colored::*;
use std::path::PathBuf;

mod analyzer;
mod dependencies;
mod reporter;
mod scanner;

use analyzer::CodeAnalyzer;
use reporter::AnalysisReport;

#[derive(Debug, Copy, Clone, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Parser, Debug)]
#[command(
    name = "cargo-rust-unused",
    about = "Detect unused code in Rust projects",
    version
)]
struct Cli {
    #[arg(long, default_value = ".", help = "Path to the Rust project")]
    path: PathBuf,

    #[arg(
        long,
        value_enum,
        default_value = "text",
        help = "Output format (text or json)"
    )]
    format: OutputFormat,

    #[arg(long, help = "Include private items in the analysis")]
    include_private: bool,
}

fn main() -> Result<()> {
    env_logger::init();

    // Remove "rust-unused" from args if called as cargo subcommand
    let args: Vec<String> = std::env::args().collect();
    let args = if args.get(1).map(|s| s == "rust-unused").unwrap_or(false) {
        args.iter()
            .take(1)
            .chain(args.iter().skip(2))
            .cloned()
            .collect()
    } else {
        args
    };

    let cli = Cli::parse_from(args);

    println!("{}", "🔍 Analyzing project...".bright_blue());

    let mut analyzer = CodeAnalyzer::new();
    analyzer.analyze_project(&cli.path)?;

    match cli.format {
        OutputFormat::Text => {
            print_text_report(&analyzer.report);
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&analyzer.report)?);
        }
    }

    Ok(())
}

fn print_text_report(report: &AnalysisReport) {
    if !report.unused_dependencies.is_empty() {
        println!("\n{}", "Unused Dependencies:".yellow());
        for dep in &report.unused_dependencies {
            println!("  - {}", dep);
        }
    }

    if !report.unused_functions.is_empty() {
        println!("\n{}", "Unused Functions:".yellow());
        for func in &report.unused_functions {
            println!("  - {}", func);
        }
    }

    if !report.unused_modules.is_empty() {
        println!("\n{}", "Unused Modules:".yellow());
        for module in &report.unused_modules {
            println!("  - {}", module);
        }
    }

    if report.unused_dependencies.is_empty()
        && report.unused_functions.is_empty()
        && report.unused_modules.is_empty()
    {
        println!("\n{}", "✨ No unused code found!".green());
    }
}
