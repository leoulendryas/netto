use git2::Repository;
use std::collections::HashMap;
use std::cmp::Reverse;
use chrono::{DateTime, Utc, TimeZone, NaiveDate};
use crate::error::CodescanError;
use std::path::Path;

pub struct AuthorStats {
    pub name: String,
    pub lines_added: usize,
    pub lines_deleted: usize,
    pub commit_count: usize,
}

pub struct GitInsight {
    pub user_authorship_stats: AuthorStats,
    pub _project_start: DateTime<Utc>,
    pub _daily_activity: HashMap<NaiveDate, usize>, // Date -> Commit count
    pub streak: usize,
    pub total_commits: usize,
    pub most_changed_files: Vec<(String, usize)>,
}

pub fn get_git_insights(path: &Path, user_name: Option<&str>) -> Result<GitInsight, CodescanError> {
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head().map_err(|_| CodescanError::NotAGitRepo)?;

    let mut daily_activity: HashMap<NaiveDate, usize> = HashMap::new();
    let mut first_commit_time = Utc::now();
    let mut commit_count = 0;
    let mut file_changes: HashMap<String, usize> = HashMap::new();
    
    // Auth detection
    let target_author = user_name.map(|s| s.to_string())
        .or_else(|| {
            let config = repo.config().ok()?;
            config.get_string("user.name").ok()
        })
        .unwrap_or_default();

    let mut user_stats = AuthorStats {
        name: target_author.clone(),
        lines_added: 0,
        lines_deleted: 0,
        commit_count: 0,
    };

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        commit_count += 1;

        let author = commit.author();
        let name = author.name().unwrap_or("");
        
        let seconds = commit.time().seconds();
        let dt = Utc.timestamp_opt(seconds, 0).unwrap();
        if dt < first_commit_time {
            first_commit_time = dt;
        }

        let date = dt.date_naive();
        *daily_activity.entry(date).or_insert(0) += 1;

        let is_user = name == target_author;
        if is_user {
            user_stats.commit_count += 1;
        }

        // Diff for lines and file changes
        let tree = commit.tree()?;
        let parent_tree = commit.parent(0).ok().and_then(|p| p.tree().ok());
        let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)?;
        
        if is_user {
            let stats = diff.stats()?;
            user_stats.lines_added += stats.insertions();
            user_stats.lines_deleted += stats.deletions();
        }

        diff.foreach(&mut |delta, _| {
            if let Some(path) = delta.new_file().path() {
                let path_str = path.to_string_lossy().to_string();
                *file_changes.entry(path_str).or_insert(0) += 1;
            }
            true
        }, None, None, None)?;
    }

    let mut sorted_files: Vec<_> = file_changes.into_iter().collect();
    sorted_files.sort_by_key(|&(_, count)| Reverse(count));
    let most_changed_files = sorted_files.into_iter().take(5).collect();

    let streak = calculate_streak(&daily_activity);

    Ok(GitInsight {
        user_authorship_stats: user_stats,
        _project_start: first_commit_time,
        _daily_activity: daily_activity,
        streak,
        total_commits: commit_count,
        most_changed_files,
    })
}

fn calculate_streak(activity: &HashMap<NaiveDate, usize>) -> usize {
    let mut current_streak = 0;
    let mut today = Utc::now().date_naive();
    
    // Check if there was activity today or yesterday to start the streak count
    if !activity.contains_key(&today) {
        today = today.pred_opt().unwrap();
        if !activity.contains_key(&today) {
            return 0;
        }
    }

    while activity.contains_key(&today) {
        current_streak += 1;
        today = today.pred_opt().unwrap();
    }

    current_streak
}
