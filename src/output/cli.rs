use crate::stats::AggregatedStats;
use colored::*;
use chrono::Utc;

pub fn display_stats(stats: &AggregatedStats) {
    println!("\n{}", "📊 Codebase Analytics".bold().cyan());
    println!("{}", "=====================".cyan());

    // User vs World
    println!("\n{}", "Your Code vs The World".bold());
    let user = &stats.user_vs_vendor.0;
    let world = &stats.user_vs_vendor.1;
    let total = user.code + world.code;
    
    if total > 0 {
        let user_perc = (user.code as f64 / total as f64) * 100.0;
        let world_perc = (world.code as f64 / total as f64) * 100.0;
        
        println!("  You:   {} lines ({:.1}%)", user.code.to_string().green(), user_perc);
        println!("  World: {} lines ({:.1}%)", world.code.to_string().yellow(), world_perc);
        
        // Visual bar
        let width = 40;
        let user_chars = (user_perc / 100.0 * width as f64) as usize;
        let world_chars = width - user_chars;
        print!("  [");
        print!("{}", "=".repeat(user_chars).green());
        print!("{}", "=".repeat(world_chars).yellow());
        println!("]");
    }

    // Languages
    println!("\n{}", "Languages".bold());
    let mut langs: Vec<_> = stats.language_stats.iter().collect();
    langs.sort_by_key(|(_, s)| std::cmp::Reverse(s.code));

    for (lang, s) in langs {
        println!("  {:<12} {:>8} lines", lang.as_str().blue(), s.code);
    }

    // Git Stats
    if let Some(git) = &stats.git_stats {
        println!("\n{}", "Git History".bold());
        let age = Utc::now().signed_duration_since(git.project_start);
        println!("  Project Age:   {} days", age.num_days().to_string().green());
        println!("  Peak Day:      {}", git.peak_day.yellow());
        println!("  Peak Hour:     {}h", git.peak_hour.to_string().yellow());
        println!("  Avg Com/Week:  {:.1}", git.avg_commits_per_week);
        println!("  Biggest Commit: {} ({} changes)", git.biggest_commit.0.dimmed(), git.biggest_commit.1);
        println!("  Problem Child: {}", git.most_changed_file.0.red());
    }

    // Fun Stats
    println!("\n{}", "Fun Stats".bold());
    if let Some(git) = &stats.git_stats {
        let days = Utc::now().signed_duration_since(git.project_start).num_days().max(1);
        let loc_per_day = stats.user_vs_vendor.0.code as f64 / days as f64;
        println!("  You write an average of {:.1} lines/day", loc_per_day.to_string().green());
        
        if let Some(count) = git.rewrite_counts.get("auth.rs") {
            println!("  You've rewritten auth.rs {} times", count.to_string().red());
        }
    }
    
    println!("");
}
