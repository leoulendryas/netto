use colored::*;

pub fn print_milestone(lines: u64) {
    let (icon, msg) = match lines {
        0..=999 => return,
        1000..=4999 => ("🥉", "1K Lines! The journey begins."),
        5000..=9999 => ("🥈", "5K Lines! You're building something real."),
        10000..=49999 => ("🥇", "10K Lines! A true craftsman."),
        _ => ("🏆", "Legendary! Your codebase is a monument."),
    };

    println!("\n{} {} {}", icon, msg.bold().yellow(), icon);
    println!("{}", r#"
       ___________
      '._==_==_=_.'
      .-\:      /-.
     | (|:.     |) |
      '-|:.     |-'
        \::.    /
         '::. .'
           ) (
         _.' '._
        `-------`
    "#.yellow());
}
