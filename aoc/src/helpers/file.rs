use itertools::Itertools;
use std::str::Lines;

#[derive(Debug)]
pub struct Input {
    text: String,
}

impl Input {
    /// Create an input from a literal string.
    /// Useful for tests.
    pub fn from_literal(input: &str) -> Input {
        let text = input.trim().to_string();
        Input { text }
    }

    /// Create an input from the day's puzzle input.
    /// This function infers the year and day from the binary name.
    pub fn from_puzzle() -> Input {
        let exe_path = std::env::current_exe().expect("Failed to get current executable path");
        let bin_name = exe_path.file_stem().expect("not a file").to_str().unwrap();
        let (year, day) = bin_name
            .split("_day")
            .collect_tuple()
            .expect("puzzle input name should be YYYY_dayDD");
        let input_path = format!("input/{year}/day{day}/input.aoc");
        let text = std::fs::read_to_string(&input_path)
            .expect("file should be a string")
            .trim()
            .to_string();
        Input { text }
    }

    /// Return the input line by line
    pub fn lines(&self) -> Lines<'_> {
        self.text.lines()
    }

    /// Splits the input by double newlines.
    pub fn groups(&self) -> Vec<&str> {
        self.text.split("\n\n").collect()
    }

    /// Tries to parse every line into a specific type T.
    /// Usage: input.parse_lines::<u32>()
    pub fn parse_lines<T>(&self) -> Vec<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        self.text
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.parse().expect("Failed to parse line"))
            .collect()
    }
}
