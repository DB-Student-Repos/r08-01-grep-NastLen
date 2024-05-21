use std::fs::File;
use std::io::{BufReader, BufRead, Error};

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug)]
pub struct Flags{
    // Should contain a flag -n Print the line numbers of each matching line.
    pub line_numbers: bool,
    // Should contain a flag -l Print only the names of files with matching lines.
    pub print_file_names: bool,
    pub case_insensitive: bool,
    pub invert_match: bool,
    pub match_entire_line: bool,
    pub invert: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
       Flags {
           line_numbers: flags.contains(&"-n"),
           print_file_names: flags.contains(&"-l"),
           case_insensitive: flags.contains(&"-i"),
           invert_match: flags.contains(&"-v"),
           match_entire_line: flags.contains(&"-x"),
           invert: flags.contains(&"-v"),
       }
    }
}


pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut results = Vec::new();

    for &file_name in files {
        let file = File::open(file_name)?;
        let reader = BufReader::new(file);

        let mut file_has_match = false;
        for (index, line) in reader.lines().enumerate() {
            let line = line?;
            let line_number = index + 1;
        
            if line_matches(&line, pattern, flags) {
                if flags.print_file_names {
                    if !file_has_match {
                        results.push(file_name.to_string());
                        file_has_match = true;
                    }
                } else {
                    let result = format_result(file_name, line_number, &line, flags, files);
                    results.push(result);
                }
            }
        }
    }

    Ok(results)
}

fn line_matches(line: &str, pattern: &str, flags: &Flags) -> bool {
    let line = if flags.case_insensitive { line.to_lowercase() } else { line.to_string() };
    let pattern = if flags.case_insensitive { pattern.to_lowercase() } else { pattern.to_string() };

    let matches = if flags.match_entire_line {
        line == pattern
    } else {
        line.contains(&pattern)
    };

    if flags.invert_match {
        !matches
    } else {
        matches
    }
}

fn format_result(file_name: &str, line_number: usize, line: &str, flags: &Flags, files: &[&str]) -> String {
    if flags.print_file_names {
        file_name.to_string()
    } else if flags.line_numbers {
        if files.len() > 1 {
            format!("{}:{}:{}", file_name, line_number, line)
        } else {
            format!("{}:{}", line_number, line)
        }
    } else {
        if files.len() > 1 {
            format!("{}:{}", file_name, line)
        } else {
            line.to_string()
        }
    }
}