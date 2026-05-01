use git2::Repository;
use std::collections::HashMap;
use chrono::{DateTime, Utc, TimeZone, Datelike, Timelike};
use std::path::PathBuf;

pub struct GitStats {
    pub project_start: DateTime<Utc>,
    pub peak_day: String,
    pub peak_hour: u32,
    pub avg_commits_per_week: f64,
    pub biggest_commit: (String, usize), // (Commit ID, lines changed)
    pub most_changed_file: (String, usize), // (File path, times changed)
    pub rewrite_counts: HashMap<String, usize>, // For fun stats like "rewrote auth.rs 14 times"
}

pub fn get_git_stats(repo_path: &PathBuf) -> Result<GitStats, git2::Error> {
    let repo = Repository::open(repo_path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let mut first_commit_time = Utc::now();
    let mut day_counts = HashMap::new();
    let mut hour_counts = HashMap::new();
    let mut file_change_counts = HashMap::new();
    let mut commit_count = 0;
    
    let mut biggest_commit_id = String::new();
    let mut max_changes = 0;

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        commit_count += 1;

        let seconds = commit.time().seconds();
        let dt = Utc.timestamp_opt(seconds, 0).unwrap();
        
        if dt < first_commit_time {
            first_commit_time = dt;
        }

        let day = format!("{:?}", dt.weekday());
        *day_counts.entry(day).or_insert(0) += 1;
        *hour_counts.entry(dt.hour()).or_insert(0) += 1;

        // Diff analysis for biggest commit and file changes
        if let Ok(parent) = commit.parent(0) {
            let tree = commit.tree()?;
            let parent_tree = parent.tree()?;
            let diff = repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), None)?;
            
            let stats = diff.stats()?;
            let changes = stats.insertions() + stats.deletions();
            if changes > max_changes {
                max_changes = changes;
                biggest_commit_id = commit.id().to_string();
            }

            diff.foreach(&mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    let path_str = path.to_string_lossy().to_string();
                    *file_change_counts.entry(path_str).or_insert(0) += 1;
                }
                true
            }, None, None, None)?;
        }
    }

    let peak_day = day_counts.into_iter().max_by_key(|&(_, count)| count).map(|(day, _)| day).unwrap_or_default();
    let peak_hour = hour_counts.into_iter().max_by_key(|&(_, count)| count).map(|(hour, _)| hour).unwrap_or(0);
    
    let duration = Utc::now().signed_duration_since(first_commit_time);
    let weeks = duration.num_weeks().max(1) as f64;
    let avg_commits_per_week = commit_count as f64 / weeks;

    let most_changed_file = file_change_counts.iter()
        .max_by_key(|&(_, count)| count)
        .map(|(path, count)| (path.clone(), *count))
        .unwrap_or_default();

    Ok(GitStats {
        project_start: first_commit_time,
        peak_day,
        peak_hour,
        avg_commits_per_week,
        biggest_commit: (biggest_commit_id, max_changes),
        most_changed_file,
        rewrite_counts: file_change_counts,
    })
}
