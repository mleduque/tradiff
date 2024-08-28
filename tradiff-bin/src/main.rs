
use std::collections::HashSet;
use std::fs::read;

use anyhow::{bail, Result};
use args::Cli;
use clap::Parser;
use diff::Diff;
use encoding_rs::Encoding;
use itertools::Itertools;
use lalrpop_util::ParseError;
use line_position::LinePosition;
use nu_ansi_term::Color;
use termsize::Size;
use tradiff_lib::{parse_trafile, LexError, Token, TraEntry};

mod args;
mod line_position;

const ORANGE: Color = Color::Rgb(255, 165, 0);

fn main() -> Result<()>{


    let args = Cli::parse();

    let first_path = &args.file1;
    let second_path = &args.file2;

    let (charset1, charset2) = match (&args.charset, &args.charset1, &args.charset2) {
        (None, None, None) => (encoding_rs::UTF_8, encoding_rs::UTF_8),
        (Some(charset), None, None) => {
            let same = match Encoding::for_label(charset.as_bytes()) {
                Some(charset) => charset,
                None => bail!("Invalid charset label {charset}.\nSee https://encoding.spec.whatwg.org/#concept-encoding-get for valid values"),
            };
            (same, same)
        },
        (None, Some(charset1), Some(charset2)) => {
            let charset1 = match Encoding::for_label(charset1.as_bytes()) {
                Some(charset) => charset,
                None => bail!("Invalid charset label {charset1}.\nSee https://encoding.spec.whatwg.org/#concept-encoding-get for valid values"),
            };
            let charset2 = match Encoding::for_label(charset2.as_bytes()) {
                Some(charset) => charset,
                None => bail!("Invalid charset label {charset2}.\nSee https://encoding.spec.whatwg.org/#concept-encoding-get for valid values"),
            };
            (charset1, charset2)
        }
        _ => bail!("Developer error, could not determine charset combination"),
    };

    let first_bytes = read(first_path).unwrap();
    let (first, replacements) = charset1.decode_without_bom_handling(&first_bytes);
    if replacements {
        println!("🚨 {} The first file ({first_path}) contains characters that could not be handled properly (replaced with �)",
                ORANGE.paint("WARN"));
    }
    let first_content = match parse(&first, "first", first_path) {
        Ok(result) => result,
        Err(error) => {
            bail!("💥 {} The first file ({first_path}) could not be parsed\n  - {:?}",
                    Color::Red.paint("ERROR"), error);
        }
    };

    let second_bytes = read(second_path).unwrap();
    let (second, replacements) = charset2.decode_without_bom_handling(&second_bytes);
    if replacements {
        println!("🚨 {} The second file ({second_path}) contains characters that could not be handled properly (replaced with �)",
                ORANGE.paint("WARN"));
    }
    let second_content = match parse(&second, "second", second_path) {
        Ok(result) => result,
        Err(error) => {
            bail!("💥 {} The second file ({second_path}) could not be parsed\n  - {:?}",
                    Color::Red.paint("ERROR"), error);
        }
    };

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
    println!("\n");
    Ok(())
}

fn parse(content: &str, qualifier: &str, path: &str) -> Result<Vec<TraEntry>> {
    let mut errors = Vec::new();

    // only keep entries, sort by id
    let parsed = match parse_trafile(&mut errors, content) {
        Ok(parsed) => parsed,
        Err(ref error) => {
            let message = process_parse_error(error, content);

            println!("🚨 {} Failed to parse the {qualifier} file ({})\n  {}",
                        Color::Red.paint("ERROR"), path, message);
            bail!("Parsing error")
        }
    };
    let mut entries = parsed
        .iter()
        .filter_map(|frag| frag.as_entry())
        .cloned()
        .collect::<Vec<_>>();
    entries.sort_by(|frag1, frag2| frag1.id.cmp(&frag2.id));


    if !errors.is_empty() {
        println!("🚨 {} The {qualifier} file ({}) contains syntax errors\n  - {}",
                Color::Red.paint("ERROR"), path, errors.iter().map(|error|
                    process_parse_error(&error.error, content)
                ).join("\n  - "))
    }

    Ok(entries)
}

fn process_parse_error(error: &ParseError<usize, Token, LexError>, source: &str) -> String {
    match error {
        ParseError::InvalidToken { location } => {
            let line_position = LinePosition::from_offset(source, *location);
            format!("Invalid token found at {line_position:?}")
        },
        ParseError::UnrecognizedEof { location: _, expected } => {
            format!("Reached the end of file but there is missing (expected) content\n  expected one of{expected:?}")
        },
        ParseError::UnrecognizedToken { token, expected } => {
            let line_position = LinePosition::from_offset(source, token.0);
            let token = &token.1;
            format!("Unrecognized token {token:?} found at {line_position:?}\n  expected one of{expected:?}")
        },
        ParseError::ExtraToken { token } => {
            let line_position = LinePosition::from_offset(source, token.0);
            let token = &token.1;
            format!("Extra token {token:?} found at {line_position:?}")
        },
        ParseError::User { error } => format!("{error:?}"),
    }
}
