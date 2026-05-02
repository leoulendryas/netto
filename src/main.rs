mod error;
mod walker;
mod loc;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "codescan",
    about = "Find out how much of your codebase you actually wrote",
    long_about = "
codescan analyzes your codebase and shows you:
  • How many lines YOU wrote vs frameworks/dependencies
  • Language breakdown with percentages  
  • Git history stats (peak hours, most changed files, etc.)
  • Fun metrics that are actually worth sharing
    ",
    version,
    author
)]
pub struct Cli {
    #[arg(default_value = ".")]
    pub path: PathBuf,

    #[arg(long, short = 'w')]
    pub web: bool,

    #[arg(long, short = 'j')]
    pub json: bool,

    #[arg(long)]
    pub no_git: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let root = cli.path.canonicalize().map_err(|_| {
        anyhow::anyhow!("Path '{}' does not exist", cli.path.display())
    })?;

    println!(
        "\n{} {}\n",
        "Scanning".bold(),
        root.display().to_string().cyan()
    );

    let files = walker::walk(&root)?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message(format!("Counting lines across {} files...", files.len()));
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    let counts = loc::count_files(&files)?;

    spinner.finish_and_clear();

    println!("Total source lines: {}", counts.total_source_lines);
    println!("Total all lines:    {}", counts.total_all_lines);
    println!("\nBy language:");
    for (lang, stats) in &counts.by_language {
        println!(
            "  {:12} {:>6} code  {:>5} comments  {:>5} blanks",
            lang.display_name(),
            stats.code,
            stats.comments,
            stats.blanks
        );
    }

    Ok(())
}
