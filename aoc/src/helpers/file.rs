use std::str::Lines;

use crate::{get_puzzle_input, helpers::grid::GridInput};

#[derive(Debug)]
pub struct Input {
    text: String,
}

impl Input {
    /// Create an input from a literal string.
    /// Useful for tests.
    pub fn from_literal(input: &str) -> Self {
        let text = input.trim().to_string();
        Self { text }
    }

    /// Create an input from the day's puzzle input.
    /// This function infers the year and day from the binary name.
    pub fn from_puzzle() -> Self {
        let text = get_puzzle_input();
        Self { text }
    }
}

impl Input {
    /// Convert this input to a GridInput
    pub fn to_grid(&self) -> GridInput {
        GridInput::new(&self.text)
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
