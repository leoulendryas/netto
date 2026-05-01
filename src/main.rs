mod walker;
mod loc;
mod stats;
mod output;

use clap::Parser;
use std::path::PathBuf;
use indicatif::ProgressBar;
use crate::stats::AggregatedStats;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value = ".")]
    path: PathBuf,
}

fn main() {
    // Initializing the analytics engine
    let args = Args::parse();
    let root = args.path.canonicalize().unwrap_or(args.path.clone());

    println!("Scanning project at {}...", root.display());

    let files = walker::walk_project(&root);
    let pb = ProgressBar::new(files.len() as u64);
    
    let mut stats = AggregatedStats::new();

    for file_info in files {
        if let Ok(line_stats) = loc::counter::count_lines(&file_info.path) {
            stats.add_file(&file_info, line_stats);
        }
        pb.inc(1);
    }
    pb.finish_and_clear();

    // Get Git stats
    if let Ok(git_stats) = loc::git::get_git_stats(&root) {
        stats.git_stats = Some(git_stats);
    }

    output::cli::display_stats(&stats);
}
