use std::collections::HashSet;

use diff::Diff;
use itertools::Itertools;
use nu_ansi_term::Color;
use termsize::Size;
use tradiff_lib::{TraEntry, TraFileParser};

const USAGE: &'static str = r#"
Usage:
    tradiff <file1> <file2>

Shows differences in entries between two weidu TRA files
"#;

const ORANGE: Color = Color::Rgb(255, 165, 0);

fn main() {

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("{}", USAGE);
        return;
    }
    let first_path = &args[1];
    let second_path = &args[2];
    let first = std::fs::read_to_string(first_path).unwrap();
    let second = std::fs::read_to_string(second_path).unwrap();

    let first_content = parse(&first);
    let second_content = parse(&second);

    let first_counts = first_content.iter().counts_by(|item| item.id);
    let second_counts = second_content.iter().counts_by(|item| item.id);

    let first_dups = first_counts.iter()
        .filter(|(_, count)| **count > 1)
        .sorted_by(|(id1, _), (id2, _)| id1.cmp(id2))
        .collect::<Vec<_>>();
    let second_dups = second_counts.iter()
        .filter(|(_, count)| **count > 1)
        .collect::<Vec<_>>();

    let mut found_dups = !first_dups.is_empty() || !second_dups.is_empty();

    let term_width = termsize::get().map(| Size { rows: _, cols }| { cols })
        .unwrap_or(60);
    let term_width = usize::from(term_width);

    if found_dups { println!("\n{}", ORANGE.paint("━".repeat(term_width))) }
    if !first_dups.is_empty() {
        found_dups = true;
        println!("🚨 {} The first file ({}) contains duplicated entries\n  - {}",
                ORANGE.paint("WARN"),
                first_path, first_dups.iter().map(|(id, _)| id).join("\n  - "))
    }
    if !second_dups.is_empty() {
        found_dups = true;
        println!("🚨 {} The second file ({}) contains duplicated entries\n  - {}",
                ORANGE.paint("WARN"),
                second_path, second_dups.iter().map(|(id, _)| id).join("\n  - "))
    }
    if found_dups { println!("{}\n", ORANGE.paint("━".repeat(term_width))) }

    let first_ids = first_content.iter().map(|entry| entry.id).collect::<HashSet<_>>();
    let second_ids = second_content.iter().map(|entry| entry.id).collect::<HashSet<_>>();

    let diff = first_ids.diff(&second_ids);

    if diff.added.is_empty() && diff.removed.is_empty() {
        println!("✅ Both files contain the same entries.");
    }
    if !diff.added.is_empty() {
        println!("{} Entries in the second file but not in the first file:\n  - {}",
                Color::Green.bold().paint("+"),
                diff.added.iter().sorted().join("\n  - "));
    }
    if !diff.removed.is_empty() {
        println!("{} Entries in the first file but not in the second file:\n  - {}",
                Color::Red.bold().paint("−"),
                diff.removed.iter().sorted().join("\n  - "));
    }
    println!("\n")
}

fn parse(path: &str) -> Vec<TraEntry> {
    // only keep entries, sort by id
    let mut entries = TraFileParser::new()
        .parse(path)
        .unwrap()
        .iter()
        .filter_map(|frag| frag.as_entry())
        .cloned()
        .collect::<Vec<_>>();
    entries.sort_by(|frag1, frag2| frag1.id.cmp(&frag2.id));
    entries
}
