use crate::loc::language::Language;
use crate::loc::counter::LanguageStats;
use std::collections::HashMap;
use colored::*;

/// Colour assigned to each language in the bar chart.
/// Falls back to a neutral cyan for anything unrecognised.
fn lang_color(name: &str) -> (u8, u8, u8) {
    match name.to_lowercase().as_str() {
        "typescript" | "tsx"              => (121, 192, 255), // blue
        "javascript" | "jsx"              => (227, 179,  65), // amber
        "rust"                            => (255, 166,  87), // orange
        "python"                          => (86,  211, 100), // green
        "go"                              => (121, 192, 255), // blue
        "css" | "scss" | "sass"           => (210, 168, 255), // purple
        "html"                            => (255, 123, 114), // red
        "json" | "yaml" | "toml"         => (227, 179,  65), // amber
        "markdown" | "md"                 => (86,  211, 100), // green
        "shell" | "bash" | "sh"          => (255, 166,  87), // orange
        "sql"                             => (121, 192, 255), // blue
        _                                 => (110, 118, 129), // muted gray
    }
}

pub fn draw_language_bars(
    stats: &HashMap<Language, LanguageStats>,
    total_lines: u64,
) {
    if total_lines == 0 {
        return;
    }

    let bar_width: usize = 28;

    // Sort descending by code line count
    let mut data: Vec<(&Language, &LanguageStats)> = stats.iter().collect();
    data.sort_by(|a, b| b.1.code.cmp(&a.1.code));

    for (lang, lang_stats) in &data {
        let name = lang.display_name();
        let pct  = lang_stats.code as f64 / total_lines as f64 * 100.0;
        let fill = (pct / 100.0 * bar_width as f64).round() as usize;
        let empty = bar_width.saturating_sub(fill);

        let (r, g, b) = lang_color(&name);
        let filled_bar = "█".repeat(fill).truecolor(r, g, b);
        let empty_bar  = "░".repeat(empty).truecolor(33, 38, 45);

        println!(
            "  {:<14} {}{}  {:.1}%",
            name.truecolor(r, g, b),
            filled_bar,
            empty_bar,
            pct
        );
    }
}
