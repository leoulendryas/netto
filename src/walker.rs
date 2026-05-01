use std::path::{Path, PathBuf};
use ignore::WalkBuilder;
use crate::loc::language::Language;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub language: Language,
    pub is_user_code: bool,
}

pub fn walk_project(root: &Path) -> Vec<FileInfo> {
    let mut files = Vec::new();
    let walker = WalkBuilder::new(root)
        .hidden(false) // Show hidden files but ignore crate will handle .git if it's in gitignore
        .git_ignore(true)
        .build();

    for result in walker {
        if let Ok(entry) = result {
            if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                let path = entry.path();
                
                // Detection logic for user code vs others
                let is_user_code = is_user_code(path);
                let language = Language::from_path(path);

                if language != Language::Other {
                    files.push(FileInfo {
                        path: path.to_path_buf(),
                        language,
                        is_user_code,
                    });
                }
            }
        }
    }
    files
}

fn is_user_code(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    
    // Patterns to exclude as "not your code"
    let blacklisted_dirs = [
        "node_modules", "vendor", "target", ".git", "dist", "build", "__pycache__"
    ];

    for dir in blacklisted_dirs {
        if path_str.contains(dir) {
            return false;
        }
    }

    if path_str.ends_with(".min.js") || path_str.contains(".generated.") {
        return false;
    }

    true
}
