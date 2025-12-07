use anyhow::Result;
use colored::*;
use regex::Regex;
use std::sync::OnceLock;

/// Extracts all signed integers from a string.
/// Example: "x=10, y=-5" -> [10, -5]
pub fn extract_numbers(input: &str) -> Vec<isize> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"-?\d+").unwrap());

    re.find_iter(input)
        .filter_map(|m| m.as_str().parse().ok())
        .collect()
}

/// Same as above, but for unsigned integers (u64).
pub fn extract_u64s(input: &str) -> Vec<u64> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(r"\d+").unwrap());

    re.find_iter(input)
        .filter_map(|m| m.as_str().parse().ok())
        .collect()
}

pub trait ParseOps {
    /// A trait to allow parsing directly on strings
    /// Usage: "10, -5".nums() -> vec![10,-5]
    fn nums(&self) -> Vec<isize>;

    /// Split into individual characters
    /// Usage: "12345".digits() -> vec![1,2,3,4,5]
    fn digits(&self) -> Vec<u32>;

    /// Paint the specified indexes RED.
    fn colour_by_index(&self, indexes: &[usize]) -> String;
}

impl ParseOps for str {
    fn nums(&self) -> Vec<isize> {
        extract_numbers(self)
    }

    fn digits(&self) -> Vec<u32> {
        self.chars().map(|c| c.to_digit(10).unwrap()).collect()
    }

    fn colour_by_index(&self, indexes: &[usize]) -> String {
        let mut output = String::new();

        for (idx, ch) in self.chars().enumerate() {
            let s = ch.to_string();
            if indexes.contains(&idx) {
                output.push_str(&format!("{}", s.red()));
            } else {
                output.push_str(&format!("{}", s.green()));
            }
        }

        output
    }
}

pub trait CombineOps {
    /// Combine two numbers into a single number
    fn combine(&self) -> Result<usize>;
}

impl CombineOps for (u32, u32) {
    fn combine(&self) -> Result<usize> {
        Ok(format!("{}{}", self.0, self.1).parse()?)
    }
}
