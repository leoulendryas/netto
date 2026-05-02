mod error;
mod walker;
mod loc;
mod output;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "netto",
    about = "Find out how much of your codebase you actually wrote",
    version,
    author
)]
pub struct Cli {
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Point to a fresh scaffold to subtract it from the count
    #[arg(long)]
    pub baseline: Option<PathBuf>,

    #[arg(long, short = 'w')]
    pub web: bool,

    #[arg(long, short = 'j')]
    pub json: bool,

    #[arg(long)]
    pub no_git: bool,

    /// Filter git stats by this author name
    #[arg(long)]
    pub author: Option<String>,
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

    // Detect frameworks
    let frameworks = walker::detect_frameworks(&root);

    // Baseline if provided
    let mut baseline_stats = None;
    if let Some(bl_path) = &cli.baseline {
        let bl_root = bl_path.canonicalize()?;
        let bl_files = walker::walk(&bl_root)?;
        baseline_stats = Some(loc::count_files(&bl_files, None)?);
    }

    let start_time = std::time::Instant::now();
    let files = walker::walk(&root)?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message(format!("Analyzing {} files...", files.len()));
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    let counts = loc::count_files(&files, baseline_stats.as_ref())?;

    let git_insights = if !cli.no_git {
        loc::get_git_insights(&root, cli.author.as_deref()).ok()
    } else {
        None
    };

    spinner.finish_and_clear();
    let duration = start_time.elapsed();

    // "I wrote this" score calculation
    let original_lines = if let Some(git) = &git_insights {
        (git.user_authorship_stats.lines_added as u64).min(counts.total_source_lines)
    } else {
        counts.total_source_lines
    };

    let project_path_str = root.to_string_lossy();
    output::cli::display_full_stats(
        &counts, 
        git_insights.as_ref(), 
        &frameworks, 
        original_lines,
        duration,
        &project_path_str
    );

    Ok(())
}
