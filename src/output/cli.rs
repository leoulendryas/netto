use crate::loc::counter::ProjectCount;
use crate::loc::git::GitInsight;
use crate::output::{charts, badges};
use colored::*;
use std::time::Duration;

const WIDTH: usize = 60;

fn divider() {
    println!("{}", "─".repeat(WIDTH).truecolor(33, 38, 45));
}

fn section_head(title: &str) {
    println!("\n{}", title.to_uppercase().truecolor(100, 110, 120));
}

pub fn display_full_stats(
    counts: &ProjectCount,
    git: Option<&GitInsight>,
    frameworks: &[&str],
    original_lines: u64,
    scan_duration: Duration,
    project_path: &str,
) {
    // ── Header ────────────────────────────────────────────────
    println!();
    println!(
        "{}",
        "NETTO".bold().truecolor(121, 192, 255)
    );
    println!(
        "{}  ·  scanned in {:.2}s",
        project_path.truecolor(110, 118, 129),
        scan_duration.as_secs_f64()
            .to_string()
            .truecolor(110, 118, 129)
    );

    // ── Detected frameworks ───────────────────────────────────
    if !frameworks.is_empty() {
        println!();
        let pills: Vec<String> = frameworks
            .iter()
            .map(|f| format!("[ {} ]", f).truecolor(121, 192, 255).to_string())
            .collect();
        println!("{}", pills.join("  "));
    }

    // ── Core counts ───────────────────────────────────────────
    divider();
    section_head("core counts");
    println!();

    let total_src = counts.total_source_lines;
    let total_all = counts.total_all_lines;
    let skipped   = counts.generated_files_count;

    // Three cards side-by-side
    println!(
        "  {:>10}   {:>10}   {:>10}",
        total_src.to_string().bold().truecolor(240, 246, 252),
        total_all.to_string().bold().truecolor(110, 118, 129),
        skipped.to_string().bold().truecolor(227, 179, 65),
    );
    println!(
        "  {:<12} {:<13} {:<13}",
        "source lines".truecolor(110, 118, 129),
        "all lines".truecolor(110, 118, 129),
        "files skipped".truecolor(110, 118, 129),
    );

    // ── Language breakdown ────────────────────────────────────
    divider();
    section_head("language breakdown");
    println!();
    charts::draw_language_bars(&counts.by_language, total_src);

    // ── I wrote this ──────────────────────────────────────────
    divider();
    section_head("i wrote this");
    println!();

    if total_src > 0 {
        let score = (original_lines as f64 / total_src as f64) * 100.0;
        let framework_lines = total_src.saturating_sub(original_lines);

        // Big percentage
        let score_str = format!("{:.1}%", score);
        println!("  {}", score_str.bold().truecolor(86, 211, 100));
        println!("  {}", "original authorship".truecolor(110, 118, 129));
        println!();

        println!(
            "  {:<24} {}",
            "your lines".truecolor(110, 118, 129),
            original_lines.to_string().truecolor(86, 211, 100)
        );
        println!(
            "  {:<24} {}",
            "framework baseline".truecolor(110, 118, 129),
            framework_lines.to_string().truecolor(110, 118, 129)
        );
        println!();

        let verdict = if score < 30.0 {
            "humbling — lots of boilerplate out there"
        } else if score < 60.0 {
            "decent — a solid mix of yours and scaffolding"
        } else {
            "impressive — most of this is hand-crafted logic"
        };
        println!("  {}", verdict.truecolor(110, 118, 129));
    }

    // ── Git insights ──────────────────────────────────────────
    if let Some(insight) = git {
        divider();
        section_head("git insights");
        println!();

        // Two stat cards
        let streak_str = format!("{} days", insight.streak);
        let commits_str = insight.total_commits.to_string();
        println!(
            "  {:>10}   {:>10}",
            streak_str.bold().truecolor(255, 123, 114),
            commits_str.bold().truecolor(240, 246, 252),
        );
        println!(
            "  {:<14} {:<14}",
            "commit streak".truecolor(110, 118, 129),
            "total commits".truecolor(110, 118, 129),
        );
        println!();

        let stats = &insight.user_authorship_stats;
        println!(
            "  {:<24} {}",
            "you added".truecolor(110, 118, 129),
            format!("+{} lines", stats.lines_added)
                .truecolor(86, 211, 100)
        );
        println!(
            "  {:<24} {}",
            "authorship".truecolor(110, 118, 129),
            stats.name.truecolor(240, 246, 252)
        );

        // Problem children
        if !insight.most_changed_files.is_empty() {
            println!();
            println!("{}", "  PROBLEM CHILDREN".truecolor(100, 110, 120));
            println!();

            let max_count = insight
                .most_changed_files
                .iter()
                .map(|(_, c)| *c)
                .max()
                .unwrap_or(1) as f64;

            for (file, count) in &insight.most_changed_files {
                let bar_len = ((*count as f64 / max_count) * 12.0) as usize;
                let bar = "█".repeat(bar_len).truecolor(255, 123, 114);
                let pad = "░".repeat(12 - bar_len).truecolor(33, 38, 45);
                println!(
                    "  {:<35} {}{} {}×",
                    file.truecolor(255, 123, 114),
                    bar,
                    pad,
                    count.to_string().truecolor(227, 179, 65)
                );
            }
        }
    }

    // ── Milestone badge ───────────────────────────────────────
    divider();
    badges::print_milestone(total_src);

    // ── Footer hint ───────────────────────────────────────────
    println!();
    println!(
        "  {} for options  ·  {} for weekly delta",
        "netto --help".truecolor(121, 192, 255),
        "netto diff".truecolor(121, 192, 255),
    );
    println!();
}
