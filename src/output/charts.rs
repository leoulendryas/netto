use crate::loc::language::Language;
use std::collections::HashMap;
use crate::loc::counter::LanguageStats;
use colored::*;

pub fn draw_pie_chart(stats: &HashMap<Language, LanguageStats>, total_lines: u64) {
    if total_lines == 0 { return; }
    
    let mut data: Vec<_> = stats.iter()
        .map(|(lang, s)| (lang.display_name(), s.code))
        .collect();
    data.sort_by_key(|&(_, count)| std::cmp::Reverse(count));

    println!("{}", "Language Breakdown".bold());
    let width = 50;
    for (name, count) in data {
        let percentage = (count as f64 / total_lines as f64) * 100.0;
        let bar_len = (percentage / 100.0 * width as f64) as usize;
        let bar = "█".repeat(bar_len);
        println!("  {:<12} [{}] {:.1}%", name.blue(), bar.green(), percentage);
    }
}
