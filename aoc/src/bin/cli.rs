//! This is the CLI program that lets use download the day's puzzle input using the aoc-cli.
//!
//! It defaults to downloading today if we are in December, otherwise it will default to the
//! first day of the previous year's puzzles.

use anyhow::{Result, anyhow};
use chrono::{Datelike, Local};
use itertools::Itertools;
use std::{env::args, fs, process::Command};

fn main() -> Result<()> {
    println!("Merry Christmas! 🎄🎄🎄");
    let args = args().collect_vec();
    let day = args.get(1).cloned().unwrap_or_default();
    let year = args.get(2).cloned().unwrap_or_default();
    let (default_day, default_year) = get_day_and_year();
    let day = day.parse().unwrap_or(default_day);
    let year = year.parse().unwrap_or(default_year);

    get_puzzle_input(day, year)?;
    template::generate_day(day, year)?;

    let day_str = format!("day{day:02}");
    let bin_name = format!("{year}_{day_str}");

    println!("☃️☃️☃️");
    println!("-> Running tests for {bin_name} (Part A)");
    #[rustfmt::skip]
    let test_a_status = Command::new("cargo")
        .args(["test", "--bin", &bin_name, "--no-default-features", "--features", "a", 
            "--", "--nocapture", "test_part_a"]).status()?;

    if !test_a_status.success() {
        println!("🥶🥶🥶");
        println!("-> Part A tests failed.");
        return Ok(());
    }

    println!("✅✅✅");
    println!("-> Part A tests passed!");
    #[rustfmt::skip]
    let test_b_status = Command::new("cargo")
        .args(["test", "--bin", &bin_name, "--no-default-features", "--features", "b", 
        "--", "--nocapture", "test_part_b"]).status()?;

    if test_b_status.success() {
        println!("✅✅✅");
        println!("-> Part B tests passed!)");
        println!("-> Running main for {bin_name} (Part B)");
        #[rustfmt::skip]
        Command::new("cargo")
            .args(["run", "--bin", &bin_name, "--no-default-features", "--features", "b"])
            .status()?;
    } else {
        println!("🤔🤔🤔");
        println!("-> Running main for {bin_name} (Part A)");
        #[rustfmt::skip]
        Command::new("cargo")
            .args(["run", "--bin", &bin_name, "--no-default-features", "--features", "a"])
            .status()?;
    }

    Ok(())
}

/// Run the aoc-cli to download the puzzle input.
///
/// https://github.com/scarvalhojr/aoc-cli
fn get_puzzle_input(day: u32, year: i32) -> Result<()> {
    // create year and day folders
    let folder_path = format!("puzzles/input/{year}/day{day:02}");
    fs::create_dir_all(&folder_path)?;
    Command::new("aoc")
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
            &format!("{folder_path}/input.aoc"),
            "--puzzle-file",
            &format!("{folder_path}/puzzle.md"),
        ])
        .status()
        .map_err(|err| {
            anyhow!("🥶🥶🥶\n{err}\n-> Is the aoc-cli installed? `cargo install aoc-cli`")
        })?;
    Ok(())
}

/// Get the default day, based on today's date
fn get_day_and_year() -> (u32, i32) {
    let today = Local::now().date_naive();
    let is_valid_aoc_date = today.month() == 12 && (1..25).contains(&today.day());
    if is_valid_aoc_date {
        return (today.day(), today.year());
    }
    let last_year = today.year() - 1;
    return (1, last_year);
}

mod template {
    use anyhow::Result;
    use proc_macro2::TokenStream;
    use quote::quote;
    use std::{fs, path::Path};

    /// Generate the code template for today!
    pub fn generate_day(day: u32, year: i32) -> Result<()> {
        let day_str = format!("day{day:02}");
        let folder_path = format!("puzzles/src/bin");
        fs::create_dir_all(&folder_path)?;
        let file_path_str = format!("{folder_path}/{year}_{day_str}.rs");
        let file_path = Path::new(&file_path_str);

        if !file_path.exists() {
            println!("-> Generating template for {year} {day_str}");
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
                    const INPUT: &str = "";
                    #part_a_test
                    #part_b_test
                }
            })?);
            fs::write(file_path, code)?;
        } else {
            println!("-> File {file_path_str} already exists. Skipping generation.");
        }

        Ok(())
    }

    /// Generator for the day's function
    fn puzzle_function_template(part: &str) -> TokenStream {
        let fn_name: proc_macro2::TokenStream = format!("part_{part}").parse().unwrap();
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
        let test_fn_name: proc_macro2::TokenStream = format!("test_part_{part}").parse().unwrap();
        let fn_name: proc_macro2::TokenStream = format!("part_{part}").parse().unwrap();
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
}
