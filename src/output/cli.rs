use crate::loc::counter::ProjectCount;
use crate::loc::git::GitInsight;
use crate::output::{charts, badges};
use colored::*;

pub fn display_full_stats(
    counts: &ProjectCount, 
    git: Option<&GitInsight>, 
    frameworks: &[&str],
    original_lines: u64
) {
    println!("\n{}", "🚀 CODESCAN ANALYTICS".bold().cyan());
    println!("{}", "=".repeat(20).cyan());

    // Frameworks
    if !frameworks.is_empty() {
        println!("\n{} {}", "Detected Frameworks:".bold(), frameworks.join(", ").yellow());
    }

    // Core Counting
    println!("\n{}", "Core Counting".bold());
    println!("  Total Source Lines: {}", counts.total_source_lines.to_string().green());
    println!("  Total All Lines:    {}", counts.total_all_lines);
    println!("  Generated Files skipped: {}", counts.generated_files_count.to_string().yellow());

    // Authorship Score
    let total = counts.total_source_lines;
    if total > 0 {
        let score = (original_lines as f64 / total as f64) * 100.0;
        print!("\n{} ", "I wrote this score:".bold());
        println!("{}% original", format!("{:.1}", score).green());
        
        if score < 30.0 {
            println!("  {}", "(Humbling. Most of this is boilerplate or world-code.)".dimmed());
        } else {
            println!("  {}", "(Impressive! A lot of this is your hand-crafted logic.)".dimmed());
        }
    }

    // Pie Chart
    println!("");
    charts::draw_pie_chart(&counts.by_language, counts.total_all_lines);

    // Git Fun Layer
    if let Some(insight) = git {
        println!("\n{}", "The Fun Layer (Git)".bold());
        println!("  Streak:           {} days 🔥", insight.streak.to_string().red());
        println!("  Total Commits:    {}", insight.total_commits);
        println!("  Authorship:       {} committed {} additions", 
            insight.user_authorship_stats.name.blue(), 
            insight.user_authorship_stats.lines_added.to_string().green()
        );

        println!("\n{}", "Problem Children (Most Changed)".bold());
        for (file, count) in &insight.most_changed_files {
            println!("  - {} ({} times)", file.red(), count.to_string().yellow());
        }
    }

    // Milestone Badges
    badges::print_milestone(counts.total_source_lines);
}
