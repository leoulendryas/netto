use colored::*;

struct Milestone {
    icon: &'static str,
    label: &'static str,
    colour: (u8, u8, u8),
}

pub fn print_milestone(lines: u64) {
    let m: Option<Milestone> = match lines {
        0..=999     => None,
        1000..=4999 => Some(Milestone {
            icon:   "◈",
            label:  "1K — the journey begins",
            colour: (176, 176, 176),
        }),
        5000..=9999 => Some(Milestone {
            icon:   "◈",
            label:  "5K — you're building something real",
            colour: (121, 192, 255),
        }),
        10000..=49999 => Some(Milestone {
            icon:   "◆",
            label:  "10K — true craftsman territory",
            colour: (227, 179, 65),
        }),
        _ => Some(Milestone {
            icon:   "★",
            label:  "legendary — your codebase is a monument",
            colour: (86, 211, 100),
        }),
    };

    if let Some(m) = m {
        let (r, g, b) = m.colour;
        println!(
            "  {} {}",
            m.icon.truecolor(r, g, b),
            m.label.truecolor(r, g, b)
        );
    }
}
