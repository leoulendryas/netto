use super::language::Language;
use crate::error::CodescanError;
use crate::walker::FileEntry;
use rayon::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Default, Serialize)]
pub struct FileCount {
    pub code: u64,
    pub comments: u64,
    pub blanks: u64,
}

impl FileCount {
    pub fn total(&self) -> u64 {
        self.code + self.comments + self.blanks
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LanguageStats {
    pub file_count: u64,
    pub code: u64,
    pub comments: u64,
    pub blanks: u64,
}

impl LanguageStats {
    pub fn total(&self) -> u64 {
        self.code + self.comments + self.blanks
    }

    fn add(&mut self, count: &FileCount) {
        self.file_count += 1;
        self.code += count.code;
        self.comments += count.comments;
        self.blanks += count.blanks;
    }
}

#[derive(Debug, Serialize)]
pub struct ProjectCount {
    pub by_language: HashMap<Language, LanguageStats>,
    pub total_source_lines: u64,
    pub total_all_lines: u64,
    pub skipped_files: u64,
}

impl ProjectCount {
    pub fn source_percentage(&self) -> f64 {
        if self.total_all_lines == 0 {
            return 0.0;
        }
        (self.total_source_lines as f64 / self.total_all_lines as f64) * 100.0
    }

    pub fn dominant_language(&self) -> Option<Language> {
        self.by_language
            .iter()
            .filter(|(lang, _)| lang.is_source_language())
            .max_by_key(|(_, stats)| stats.code)
            .map(|(lang, _)| *lang)
    }
}

pub fn count_files(files: &[FileEntry]) -> Result<ProjectCount, CodescanError> {
    let results: Vec<(Language, FileCount)> = files
        .par_iter()
        .filter_map(|file| {
            let ext = file.extension.as_deref()?; 
            let language = Language::from_extension(ext)?; 
            let count = count_single_file(file, language).ok()?; 
            Some((language, count))
        })
        .collect();

    let mut by_language: HashMap<Language, LanguageStats> = HashMap::new();
    let mut total_source_lines: u64 = 0;
    let mut total_all_lines: u64 = 0;
    let skipped_files = (files.len() - results.len()) as u64;

    for (language, count) in &results {
        let stats = by_language.entry(*language).or_default();
        stats.add(count);

        total_all_lines += count.total();
        if language.is_source_language() {
            total_source_lines += count.code;
        }
    }

    Ok(ProjectCount {
        by_language,
        total_source_lines,
        total_all_lines,
        skipped_files,
    })
}

fn count_single_file(
    file: &FileEntry,
    language: Language,
) -> Result<FileCount, CodescanError> {
    let bytes = fs::read(&file.path)?;
    let content = String::from_utf8_lossy(&bytes);

    let line_prefixes = language.line_comment_prefixes();
    let block_delims = language.block_comment_delimiters();

    let mut count = FileCount::default();
    let mut in_block_comment = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            count.blanks += 1;
            continue;
        }

        if let Some((open, close)) = block_delims {
            if in_block_comment {
                count.comments += 1;
                if trimmed.contains(close) {
                    in_block_comment = false;
                }
                continue;
            }

            if trimmed.starts_with(open) {
                count.comments += 1;
                let after_open = &trimmed[open.len()..];
                if !after_open.contains(close) {
                    in_block_comment = true;
                }
                continue;
            }
        }

        if line_prefixes.iter().any(|prefix| trimmed.starts_with(prefix)) {
            count.comments += 1;
            continue;
        }

        count.code += 1;
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::walker::FileEntry;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn make_temp_file(content: &str, ext: &str) -> (NamedTempFile, FileEntry) {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", content).unwrap();

        let path = file.path().to_path_buf();
        let entry = FileEntry {
            size_bytes: content.len() as u64,
            extension: Some(ext.to_string()),
            path,
        };
        (file, entry) 
    }

    #[test]
    fn counts_rust_code_and_comments() {
        let code = r#"
// This is a comment
fn main() {
    println!("hello");
}

"#;
        let (_file, entry) = make_temp_file(code, "rs");
        let result = count_single_file(&entry, Language::Rust).unwrap();

        assert_eq!(result.comments, 1);
        assert_eq!(result.code, 3); 
        assert_eq!(result.blanks, 2);
    }

    #[test]
    fn handles_block_comments() {
        let code = "/* start\n   middle\n   end */\nlet x = 1;\n";
        let (_file, entry) = make_temp_file(code, "rs");
        let result = count_single_file(&entry, Language::Rust).unwrap();

        assert_eq!(result.comments, 3);
        assert_eq!(result.code, 1);
    }

    #[test]
    fn counts_python_hash_comments() {
        let code = "# comment\nx = 1\n\n";
        let (_file, entry) = make_temp_file(code, "py");
        let result = count_single_file(&entry, Language::Python).unwrap();

        assert_eq!(result.comments, 1);
        assert_eq!(result.code, 1);
        assert_eq!(result.blanks, 1);
    }
}
