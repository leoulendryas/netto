use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Language {
    Rust,
    TypeScript,
    JavaScript,
    Python,
    Go,
    Java,
    Json,
    Yaml,
    Markdown,
    Html,
    Css,
    Shell,
    Other,
}

impl Language {
    pub fn from_extension(ext: &str) -> Option<Self> {
        let lang = match ext {
            "rs"                        => Self::Rust,
            "ts" | "tsx"                => Self::TypeScript,
            "js" | "jsx" | "mjs"       => Self::JavaScript,
            "py" | "pyw"               => Self::Python,
            "go"                        => Self::Go,
            "java"                      => Self::Java,
            "json"                      => Self::Json,
            "yml" | "yaml"             => Self::Yaml,
            "md" | "mdx"               => Self::Markdown,
            "html" | "htm"             => Self::Html,
            "css" | "scss" | "sass"    => Self::Css,
            "sh" | "bash" | "zsh"      => Self::Shell,

            "toml" | "ini" | "env" |
            "sql" | "graphql" | "xml"  => Self::Other,

            _ => return None,
        };

        Some(lang)
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Self::Rust       => "Rust",
            Self::TypeScript => "TypeScript",
            Self::JavaScript => "JavaScript",
            Self::Python     => "Python",
            Self::Go         => "Go",
            Self::Java       => "Java",
            Self::Json       => "JSON",
            Self::Yaml       => "YAML",
            Self::Markdown   => "Markdown",
            Self::Html       => "HTML",
            Self::Css        => "CSS",
            Self::Shell      => "Shell",
            Self::Other      => "Other",
        }
    }

    pub fn line_comment_prefixes(self) -> &'static [&'static str] {
        match self {
            Self::Rust | Self::Go |
            Self::Java | Self::JavaScript |
            Self::TypeScript | Self::Css   => &["//"],

            Self::Python | Self::Shell |
            Self::Yaml | Self::Other       => &["#"],

            Self::Html                     => &[],

            Self::Json | Self::Markdown    => &[],
        }
    }

    pub fn block_comment_delimiters(self) -> Option<(&'static str, &'static str)> {
        match self {
            Self::Rust | Self::Go |
            Self::Java | Self::JavaScript |
            Self::TypeScript               => Some(("/*", "*/")),

            Self::Html                     => Some(("<!--", "-->")),

            Self::Css                      => Some(("/*", "*/")),

            Self::Python                   => Some(("\"\"\"", "\"\"\"")),

            _ => None,
        }
    }

    pub fn is_source_language(self) -> bool {
        matches!(
            self,
            Self::Rust
                | Self::TypeScript
                | Self::JavaScript
                | Self::Python
                | Self::Go
                | Self::Java
                | Self::Shell
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_rust_extension() {
        assert_eq!(Language::from_extension("rs"), Some(Language::Rust));
    }

    #[test]
    fn maps_tsx_to_typescript() {
        assert_eq!(Language::from_extension("tsx"), Some(Language::TypeScript));
    }

    #[test]
    fn returns_none_for_unknown() {
        assert_eq!(Language::from_extension("xyz"), None);
    }

    #[test]
    fn rust_is_source_language() {
        assert!(Language::Rust.is_source_language());
    }

    #[test]
    fn json_is_not_source_language() {
        assert!(!Language::Json.is_source_language());
    }
}
