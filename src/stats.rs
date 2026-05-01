use std::collections::HashMap;
use crate::loc::language::Language;
use crate::loc::counter::LineStats;
use crate::loc::git::GitStats;
use crate::walker::FileInfo;

pub struct AggregatedStats {
    pub language_stats: HashMap<Language, LineStats>,
    pub user_vs_vendor: (LineStats, LineStats), // (User, Vendor)
    pub git_stats: Option<GitStats>,
}

impl AggregatedStats {
    pub fn new() -> Self {
        Self {
            language_stats: HashMap::new(),
            user_vs_vendor: (LineStats::default(), LineStats::default()),
            git_stats: None,
        }
    }

    pub fn add_file(&mut self, info: &FileInfo, stats: LineStats) {
        let entry = self.language_stats.entry(info.language).or_default();
        entry.total += stats.total;
        entry.code += stats.code;
        entry.comment += stats.comment;
        entry.blank += stats.blank;

        if info.is_user_code {
            self.user_vs_vendor.0.total += stats.total;
            self.user_vs_vendor.0.code += stats.code;
            self.user_vs_vendor.0.comment += stats.comment;
            self.user_vs_vendor.0.blank += stats.blank;
        } else {
            self.user_vs_vendor.1.total += stats.total;
            self.user_vs_vendor.1.code += stats.code;
            self.user_vs_vendor.1.comment += stats.comment;
            self.user_vs_vendor.1.blank += stats.blank;
        }
    }
}
