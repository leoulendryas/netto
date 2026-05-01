use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Default, Clone)]
pub struct LineStats {
    pub total: usize,
    pub code: usize,
    pub comment: usize,
    pub blank: usize,
}

pub fn count_lines(path: &Path) -> io::Result<LineStats> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut stats = LineStats::default();

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue, // Skip lines that aren't valid UTF-8
        };

        stats.total += 1;
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            stats.blank += 1;
        } else if trimmed.starts_with("//") || trimmed.starts_with("#") || trimmed.starts_with("/*") || trimmed.starts_with("*") {
            // Very basic comment detection
            stats.comment += 1;
        } else {
            stats.code += 1;
        }
    }

    Ok(stats)
}
