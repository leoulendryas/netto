use crate::error::CodescanError;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub extension: Option<String>,
}

const SKIP_DIRS: &[&str] = &[
    "node_modules",
    "target",
    "vendor",
    "dist",
    "build",
    ".git",
    "__pycache__",
    ".pytest_cache",
    "venv",
    ".venv",
    "env",
    ".next",
    ".nuxt",
    "coverage",
    ".nyc_output",
    "generated",
    "gen",
    ".gradle",
    ".idea",
    ".vscode",
];

const MAX_FILE_SIZE_BYTES: u64 = 1_000_000;

pub fn walk(root: &Path) -> Result<Vec<FileEntry>, CodescanError> {
    let mut entries = Vec::new();

    let walker = WalkBuilder::new(root)
        .hidden(true)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .require_git(false)
        .build();

    for result in walker {
        let entry = result.map_err(|e| CodescanError::Walk(e.to_string()))?;

        let file_type = match entry.file_type() {
            Some(ft) => ft,
            None => continue,
        };

        if !file_type.is_file() {
            continue;
        }

        let path = entry.path().to_path_buf();

        if is_skipped_path(&path) {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let size_bytes = metadata.len();

        if size_bytes > MAX_FILE_SIZE_BYTES {
            continue;
        }

        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        entries.push(FileEntry {
            path,
            size_bytes,
            extension,
        });
    }

    Ok(entries)
}

fn is_skipped_path(path: &Path) -> bool {
    path.components().any(|component| {
        if let std::path::Component::Normal(name) = component {
            if let Some(name_str) = name.to_str() {
                return SKIP_DIRS.contains(&name_str);
            }
        }
        false
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn skips_node_modules() {
        let path = Path::new("my-project/node_modules/lodash/index.js");
        assert!(is_skipped_path(path));
    }

    #[test]
    fn skips_nested_vendor() {
        let path = Path::new("src/vendor/library/code.go");
        assert!(is_skipped_path(path));
    }

    #[test]
    fn allows_normal_source_files() {
        let path = Path::new("src/main.rs");
        assert!(!is_skipped_path(path));
    }

    #[test]
    fn allows_src_in_path() {
        let path = Path::new("src/lib/utils.ts");
        assert!(!is_skipped_path(path));
    }
}
