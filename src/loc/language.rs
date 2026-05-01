use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    JavaScript,
    TypeScript,
    Python,
    Go,
    Java,
    Other,
}

impl Language {
    pub fn from_path(path: &Path) -> Self {
        match path.extension().and_then(|s| s.to_str()) {
            Some("rs") => Language::Rust,
            Some("js") | Some("jsx") | Some("mjs") => Language::JavaScript,
            Some("ts") | Some("tsx") => Language::TypeScript,
            Some("py") => Language::Python,
            Some("go") => Language::Go,
            Some("java") => Language::Java,
            _ => Language::Other,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Rust => "Rust",
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::Python => "Python",
            Language::Go => "Go",
            Language::Java => "Java",
            Language::Other => "Other",
        }
    }
}
