//! This is the CLI program that lets use download the day's puzzle input using the aoc-cli.
//!
//! It defaults to downloading today if we are in December, otherwise it will default to the
//! first day of the previous year's puzzles.

use anyhow::{Context as _, Result, bail};
use clap::Parser;
use jiff::ZonedDateTime;
use std::{
    fs,
    path::Path,
    process::{Command, ExitStatus},
};

#[derive(Parser, Debug)]
#[command(about = "Advent of Code puzzle runner and scaffolder")]
struct Cli {
    // Puzzle day (1-25 for 2015-2024, 1-12 for 2025+)
    day: Option<u8>,

    /// Puzzle year (e.g. 2025)
    year: Option<i16>,

    /// Force re-downloading puzzle.md and input.aoc even if they exist
    #[arg(short, long)]
    refresh: bool,

    /// Run the puzzle binary in release mode
    #[arg(long)]
    release: bool,
}

fn main() -> Result<()> {
    println!("Merry Christmas! 🎄🎄🎄");
    let cli = Cli::parse();
    let (default_day, default_year) = get_day_and_year();
    let day = cli.day.unwrap_or(default_day);
    let year = cli.year.unwrap_or(default_year);

    if year < 2015 {
        bail!("Advent of Code started in 2015 (got {year}).");
    }

    let max_days = max_days_for_year(year);
    if !(1..=max_days).contains(&day) {
        bail!("Invalid day {day} for {year} (must be between 1 and {max_days}).");
    }

    get_puzzle_input(day, year, cli.refresh)?;
    template::generate_day(day, year)?;

    let day_str = format!("day{day:02}");
    let bin_name = format!("{year}_{day_str}");

    println!("☃️☃️☃️");
    println!("-> Running tests for {bin_name} (Part A)");
    let test_a_status = run_cargo(
        "test",
        &bin_name,
        "a",
        false,
        &["--", "--nocapture", "test_part_a"],
    )?;

    if !test_a_status.success() {
        println!("🥶🥶🥶");
        bail!("Part A tests failed.");
    }

    println!("✅✅✅");
    println!("-> Part A tests passed!");
    let test_b_status = run_cargo(
        "test",
        &bin_name,
        "b",
        false,
        &["--", "--nocapture", "test_part_b"],
    )?;

    if test_b_status.success() {
        println!("✅✅✅");
        println!("-> Part B tests passed!");
        println!("-> Running main for {bin_name} (Part B)");
        run_cargo("run", &bin_name, "b", cli.release, &[])?;
    } else {
        println!("🤔🤔🤔");
        println!("-> Running main for {bin_name} (Part A)");
        run_cargo("run", &bin_name, "a", cli.release, &[])?;
    }

    Ok(())
}

/// Helper to execute `cargo test` or `cargo run` for a specific puzzle binary and feature part.
fn run_cargo(
    subcommand: &str,
    bin_name: &str,
    feature: &str,
    release: bool,
    extra_args: &[&str],
) -> Result<ExitStatus> {
    let mut cmd = Command::new("cargo");
    cmd.args([
        subcommand,
        "--bin",
        bin_name,
        "--no-default-features",
        "--features",
        feature,
    ]);

    if release {
        cmd.arg("--release");
    }

    cmd.args(extra_args)
        .status()
        .with_context(|| format!("Failed to execute `cargo {subcommand}` for {bin_name}"))
}

/// Returns the maximum number of puzzle days for a given AoC year.
fn max_days_for_year(year: i16) -> u8 {
    if year >= 2025 { 12 } else { 25 }
}

/// Run aoc-cli to download the puzzle input and description.
///
/// https://github.com/scarvalhojr/aoc-cli
fn get_puzzle_input(day: u8, year: i16, refresh: bool) -> Result<()> {
    let folder_path = format!("puzzles/input/{year}/day{day:02}");
    let input_file = format!("{folder_path}/input.aoc");
    let puzzle_file = format!("{folder_path}/puzzle.md");

    if !refresh && Path::new(&input_file).exists() {
        return Ok(());
    }

    if !Path::new(".session").exists() {
        bail!("Missing `.session` file in current directory.");
    }

    fs::create_dir_all(&folder_path)?;

    let output = Command::new("aoc")
        .args([
            "download",
            "--day",
            &day.to_string(),
            "--year",
            &year.to_string(),
            "--overwrite",
            "--session-file",
            ".session",
            "--input-file",
            &input_file,
            "--puzzle-file",
            &puzzle_file,
        ])
        .output()
        .context("Failed to execute `aoc`. Is `aoc-cli` installed? (`cargo install aoc-cli`)")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        bail!(
            "`aoc download` failed (exit code {:?}):\n{}{}",
            output.status.code(),
            stderr.trim(),
            if stdout.is_empty() {
                String::new()
            } else {
                format!("\n{}", stdout.trim())
            }
        );
    }

    Ok(())
}

fn get_day_and_year() -> (u8, i16) {
    let today = ZonedDateTime::now().date();
    let year = today.year();
    let max_days = max_days_for_year(year) as i8;

    if today.month() == 12 && (1..=max_days).contains(&today.day()) {
        return (today.day() as u8, year);
    }

    (1, year - 1)
}

mod template {
    use anyhow::Result;
    use proc_macro2::TokenStream;
    use quote::{format_ident, quote};
    use std::{fs, path::Path};

    /// Generate the code template for today!
    pub fn generate_day(day: u8, year: i16) -> Result<()> {
        let day_str = format!("day{day:02}");
        let folder_path = "puzzles/src/bin";
        fs::create_dir_all(folder_path)?;
        let file_path = Path::new(folder_path).join(format!("{year}_{day_str}.rs"));

        if !file_path.exists() {
            println!("-> Generating template for {year} {day_str}");
            let input = extract_example_input(year, day);
            let part_a_fn = puzzle_function_template("a");
            let part_b_fn = puzzle_function_template("b");
            let part_a_test = puzzle_test_template("a");
            let part_b_test = puzzle_test_template("b");
            let code = prettyplease::unparse(&syn::parse2(quote! {
                use aoc::prelude::*;
                #[allow(unused)]
                #part_a_fn
                #[allow(unused)]
                #part_b_fn
                fn main() {
                    let input = Input::from_puzzle();
                    #[cfg(feature = "a")]
                    println!("part a: {}", part_a(&input).unwrap());
                    #[cfg(feature = "b")]
                    println!("part b: {}", part_b(&input).unwrap());
                }
                #[cfg(test)]
                mod test {
                    use super::*;
                    const INPUT: &str = #input;
                    #part_a_test
                    #part_b_test
                }
            })?);
            fs::write(&file_path, code)?;
        } else {
            println!(
                "-> File {} already exists. Skipping generation.",
                file_path.display()
            );
        }

        Ok(())
    }

    /// Generator for the day's function
    fn puzzle_function_template(part: &str) -> TokenStream {
        let fn_name = format_ident!("part_{part}");
        quote! {
            fn #fn_name(input: &Input) -> Result<usize> {
                let mut output = 0;
                for line in input.lines() {
                    println!("{}", line);
                }
                Ok(output)
            }
        }
    }

    /// Generator for the day's test function
    fn puzzle_test_template(part: &str) -> TokenStream {
        let test_fn_name = format_ident!("test_part_{part}");
        let fn_name = format_ident!("part_{part}");
        quote! {
            #[test]
            fn #test_fn_name() {
                let input = Input::from_literal(INPUT);
                let result = #fn_name(&input).unwrap();
                assert_eq!(result, 0);
                todo!("set up test");
            }
        }
    }

    /// Extract the first fenced code block after "For example" in Part One of puzzle.md.
    fn extract_example_input(year: i16, day: u8) -> String {
        let puzzle_path = format!("puzzles/input/{year}/day{day:02}/puzzle.md");
        let Ok(markdown) = fs::read_to_string(&puzzle_path) else {
            return String::new();
        };

        let part_one = markdown
            .split_once("## --- Part Two ---")
            .map(|(p1, _)| p1)
            .unwrap_or(&markdown);

        let search_area = part_one
            .find("For example")
            .map(|idx| &part_one[idx..])
            .unwrap_or(part_one);

        let Some(start_fence) = search_area.find("```") else {
            return String::new();
        };
        let after_open = &search_area[start_fence + 3..];
        let Some(newline_idx) = after_open.find('\n') else {
            return String::new();
        };
        let code_start = &after_open[newline_idx + 1..];
        let Some(end_fence) = code_start.find("```") else {
            return String::new();
        };

        code_start[..end_fence]
            .trim_end_matches(['\r', '\n'])
            .to_string()
    }
}
