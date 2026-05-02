use serde::Serialize;

/// Every language netto knows about.
///
/// The goal is to cover any framework someone might run netto against —
/// Flutter, Next.js, Svelte, Express, Laravel, Rails, Django, Spring,
/// Unity, and everything in between. If a file extension maps to a
/// language here, netto will count it. If not, it silently skips it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Language {
    // ── Systems ───────────────────────────────────────────────
    Rust,
    C,
    Cpp,
    CSharp,     // Unity, .NET, ASP.NET
    Zig,
    // ── JVM ───────────────────────────────────────────────────
    Java,       // Spring, Android
    Kotlin,     // Android, Ktor
    Scala,      // Spark, Play
    Groovy,     // Gradle build scripts
    // ── Web — compiled ────────────────────────────────────────
    TypeScript, // Next.js, Remix, Angular, NestJS
    JavaScript, // Express, Svelte (compiled), React, Vue
    // ── Web — templated ───────────────────────────────────────
    Html,
    Css,
    Scss,       // Sass/SCSS — separate from plain CSS
    Less,
    Svelte,     // .svelte files are HTML+JS+CSS in one
    Vue,        // .vue files — same idea
    Astro,      // .astro files — Astro framework
    Htmx,       // .html counted under Html; this covers .htmx if used
    // ── Scripting ─────────────────────────────────────────────
    Python,     // Django, FastAPI, Flask
    Ruby,       // Rails, Sinatra
    Php,        // Laravel, WordPress, Symfony
    Perl,
    Lua,        // Neovim plugins, game scripting
    // ── Mobile ────────────────────────────────────────────────
    Dart,       // Flutter
    Swift,      // iOS, macOS
    ObjectiveC, // Legacy iOS/macOS
    // ── Functional ────────────────────────────────────────────
    Haskell,
    Elixir,     // Phoenix
    Erlang,
    Clojure,
    FSharp,
    OCaml,
    // ── Go ecosystem ──────────────────────────────────────────
    Go,         // Gin, Echo, Fiber
    // ── Data / ML ─────────────────────────────────────────────
    Julia,
    R,
    Matlab,
    Notebooks,  // .ipynb — Jupyter
    // ── Infrastructure / config ───────────────────────────────
    Shell,      // bash, zsh, sh
    PowerShell,
    Dockerfile,
    // ── Query ─────────────────────────────────────────────────
    Sql,
    GraphQL,
    // ── Serialisation / config (not source, but tracked) ──────
    Json,
    Yaml,
    Toml,
    Xml,
    // ── Docs ──────────────────────────────────────────────────
    Markdown,
    Rst,        // reStructuredText — Python docs
    Latex,
    // ── Catch-all ─────────────────────────────────────────────
    Other,
}

impl Language {
    /// Map a file extension to a Language.
    /// Returns None for extensions netto doesn't recognise
    /// (lock files, images, fonts, etc.) — those files are skipped.
    pub fn from_extension(ext: &str) -> Option<Self> {
        let lang = match ext.to_lowercase().as_str() {
            // Systems
            "rs"                            => Self::Rust,
            "c" | "h"                       => Self::C,
            "cpp" | "cc" | "cxx" |
            "hpp" | "hxx" | "hh"           => Self::Cpp,
            "cs"                            => Self::CSharp,
            "zig"                           => Self::Zig,

            // JVM
            "java"                          => Self::Java,
            "kt" | "kts"                    => Self::Kotlin,
            "scala" | "sc"                  => Self::Scala,
            "groovy" | "gradle"             => Self::Groovy,

            // Web — compiled
            "ts" | "tsx" | "mts" | "cts"   => Self::TypeScript,
            "js" | "jsx" | "mjs" |
            "cjs"                           => Self::JavaScript,

            // Web — templated
            "html" | "htm" | "xhtml"       => Self::Html,
            "css"                           => Self::Css,
            "scss"                          => Self::Scss,
            "less"                          => Self::Less,
            "svelte"                        => Self::Svelte,
            "vue"                           => Self::Vue,
            "astro"                         => Self::Astro,
            "htmx"                          => Self::Htmx,

            // Scripting
            "py" | "pyw" | "pyx"           => Self::Python,
            "rb" | "rake" | "gemspec"      => Self::Ruby,
            "php" | "php3" | "php4" |
            "php5" | "phtml"               => Self::Php,
            "pl" | "pm"                     => Self::Perl,
            "lua"                           => Self::Lua,

            // Mobile
            "dart"                          => Self::Dart,
            "swift"                         => Self::Swift,
            "m" | "mm"                      => Self::ObjectiveC,

            // Functional
            "hs" | "lhs"                    => Self::Haskell,
            "ex" | "exs"                    => Self::Elixir,
            "erl" | "hrl"                   => Self::Erlang,
            "clj" | "cljs" | "cljc"        => Self::Clojure,
            "fs" | "fsx" | "fsi"           => Self::FSharp,
            "ml" | "mli"                    => Self::OCaml,

            // Go
            "go"                            => Self::Go,

            // Data / ML
            "jl"                            => Self::Julia,
            "r" | "rmd"                     => Self::R,
            "ipynb"                         => Self::Notebooks,

            // Infrastructure
            "sh" | "bash" | "zsh" |
            "fish" | "ksh"                  => Self::Shell,
            "ps1" | "psm1" | "psd1"        => Self::PowerShell,
            "dockerfile"                    => Self::Dockerfile,

            // Query
            "sql"                           => Self::Sql,
            "graphql" | "gql"              => Self::GraphQL,

            // Config / serialisation
            "json" | "jsonc"               => Self::Json,
            "yml" | "yaml"                 => Self::Yaml,
            "toml"                          => Self::Toml,
            "xml" | "xsl" | "xslt"        => Self::Xml,

            // Docs
            "md" | "mdx"                   => Self::Markdown,
            "rst"                           => Self::Rst,
            "tex" | "sty"                  => Self::Latex,

            // Generic catch-all for ini/env/conf style files
            "ini" | "env" | "conf" |
            "cfg" | "properties"           => Self::Other,

            _ => return None,
        };
        Some(lang)
    }

    /// Human-readable name shown in the output.
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Rust       => "Rust",
            Self::C          => "C",
            Self::Cpp        => "C++",
            Self::CSharp     => "C#",
            Self::Zig        => "Zig",
            Self::Java       => "Java",
            Self::Kotlin     => "Kotlin",
            Self::Scala      => "Scala",
            Self::Groovy     => "Groovy",
            Self::TypeScript => "TypeScript",
            Self::JavaScript => "JavaScript",
            Self::Html       => "HTML",
            Self::Css        => "CSS",
            Self::Scss       => "SCSS",
            Self::Less       => "Less",
            Self::Svelte     => "Svelte",
            Self::Vue        => "Vue",
            Self::Astro      => "Astro",
            Self::Htmx       => "HTMX",
            Self::Python     => "Python",
            Self::Ruby       => "Ruby",
            Self::Php        => "PHP",
            Self::Perl       => "Perl",
            Self::Lua        => "Lua",
            Self::Dart       => "Dart",
            Self::Swift      => "Swift",
            Self::ObjectiveC => "Objective-C",
            Self::Haskell    => "Haskell",
            Self::Elixir     => "Elixir",
            Self::Erlang     => "Erlang",
            Self::Clojure    => "Clojure",
            Self::FSharp     => "F#",
            Self::OCaml      => "OCaml",
            Self::Go         => "Go",
            Self::Julia      => "Julia",
            Self::R          => "R",
            Self::Matlab     => "MATLAB",
            Self::Notebooks  => "Jupyter",
            Self::Shell      => "Shell",
            Self::PowerShell => "PowerShell",
            Self::Dockerfile => "Dockerfile",
            Self::Sql        => "SQL",
            Self::GraphQL    => "GraphQL",
            Self::Json       => "JSON",
            Self::Yaml       => "YAML",
            Self::Toml       => "TOML",
            Self::Xml        => "XML",
            Self::Markdown   => "Markdown",
            Self::Rst        => "reStructuredText",
            Self::Latex      => "LaTeX",
            Self::Other      => "Other",
        }
    }

    /// Single-line comment prefix(es) for this language.
    /// Used by the counter to skip comment lines.
    pub fn line_comment_prefixes(self) -> &'static [&'static str] {
        match self {
            // // style
            Self::Rust | Self::C | Self::Cpp | Self::CSharp |
            Self::Zig  | Self::Java | Self::Kotlin | Self::Scala |
            Self::Groovy | Self::TypeScript | Self::JavaScript |
            Self::Svelte | Self::Vue | Self::Astro | Self::Go |
            Self::Swift | Self::Dart | Self::FSharp |
            Self::Css  | Self::Scss | Self::Less    => &["//"],

            // # style
            Self::Python | Self::Ruby | Self::Shell |
            Self::PowerShell | Self::Perl | Self::R |
            Self::Julia | Self::Yaml | Self::Dockerfile |
            Self::Elixir | Self::Other               => &["#"],

            // -- style
            Self::Haskell | Self::Sql | Self::Lua    => &["--"],

            // ; style
            Self::Clojure                            => &[";"],

            // % style
            Self::Erlang | Self::Latex | Self::Matlab => &["%"],

            // No line comments
            Self::Html | Self::Xml | Self::Json |
            Self::Toml | Self::Markdown | Self::Rst |
            Self::GraphQL | Self::ObjectiveC |
            Self::OCaml | Self::Htmx | Self::Php |
            Self::Notebooks                          => &[],
        }
    }

    /// Opening and closing delimiters for block comments, if the language has them.
    pub fn block_comment_delimiters(self) -> Option<(&'static str, &'static str)> {
        match self {
            Self::Rust | Self::C | Self::Cpp | Self::CSharp |
            Self::Java | Self::Kotlin | Self::Scala | Self::Groovy |
            Self::TypeScript | Self::JavaScript | Self::Go |
            Self::Swift | Self::Dart | Self::Css |
            Self::Scss | Self::Less | Self::Php      => Some(("/*", "*/")),

            Self::Html | Self::Svelte |
            Self::Astro | Self::Htmx | Self::Xml    => Some(("<!--", "-->")),

            Self::Python                             => Some(("\"\"\"", "\"\"\"")),

            Self::Haskell                            => Some(("{-", "-}")),
            Self::OCaml                              => Some(("(*", "*)")),
            Self::Lua                                => Some(("--[[", "]]")),
            Self::Erlang                             => Some(("%{", "}")), // not standard but safe

            // Vue and Astro have sections — treated as HTML for block comments
            Self::Vue                                => Some(("<!--", "-->")),

            _ => None,
        }
    }

    /// Whether this language counts toward the "source lines" total.
    ///
    /// Config, data, and doc files are tracked for the breakdown chart
    /// but don't count toward the "I wrote this" score — they inflate
    /// the number without reflecting actual programming work.
    pub fn is_source_language(self) -> bool {
        matches!(
            self,
            Self::Rust
                | Self::C
                | Self::Cpp
                | Self::CSharp
                | Self::Zig
                | Self::Java
                | Self::Kotlin
                | Self::Scala
                | Self::Groovy
                | Self::TypeScript
                | Self::JavaScript
                | Self::Svelte
                | Self::Vue
                | Self::Astro
                | Self::Htmx
                | Self::Python
                | Self::Ruby
                | Self::Php
                | Self::Perl
                | Self::Lua
                | Self::Dart
                | Self::Swift
                | Self::ObjectiveC
                | Self::Haskell
                | Self::Elixir
                | Self::Erlang
                | Self::Clojure
                | Self::FSharp
                | Self::OCaml
                | Self::Go
                | Self::Julia
                | Self::R
                | Self::Shell
                | Self::PowerShell
                | Self::Dockerfile
                | Self::Sql
                | Self::GraphQL
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_rust()       { assert_eq!(Language::from_extension("rs"),     Some(Language::Rust)); }
    #[test]
    fn maps_tsx()        { assert_eq!(Language::from_extension("tsx"),    Some(Language::TypeScript)); }
    #[test]
    fn maps_dart()       { assert_eq!(Language::from_extension("dart"),   Some(Language::Dart)); }
    #[test]
    fn maps_svelte()     { assert_eq!(Language::from_extension("svelte"), Some(Language::Svelte)); }
    #[test]
    fn maps_vue()        { assert_eq!(Language::from_extension("vue"),    Some(Language::Vue)); }
    #[test]
    fn maps_astro()      { assert_eq!(Language::from_extension("astro"),  Some(Language::Astro)); }
    #[test]
    fn maps_swift()      { assert_eq!(Language::from_extension("swift"),  Some(Language::Swift)); }
    #[test]
    fn maps_kotlin()     { assert_eq!(Language::from_extension("kt"),     Some(Language::Kotlin)); }
    #[test]
    fn maps_elixir()     { assert_eq!(Language::from_extension("ex"),     Some(Language::Elixir)); }
    #[test]
    fn maps_php()        { assert_eq!(Language::from_extension("php"),    Some(Language::Php)); }
    #[test]
    fn maps_ruby()       { assert_eq!(Language::from_extension("rb"),     Some(Language::Ruby)); }
    #[test]
    fn unknown_returns_none() { assert_eq!(Language::from_extension("xyz"), None); }
    #[test]
    fn dart_is_source()  { assert!(Language::Dart.is_source_language()); }
    #[test]
    fn json_not_source() { assert!(!Language::Json.is_source_language()); }
    #[test]
    fn yaml_not_source() { assert!(!Language::Yaml.is_source_language()); }
    #[test]
    fn svelte_is_source(){ assert!(Language::Svelte.is_source_language()); }
}
