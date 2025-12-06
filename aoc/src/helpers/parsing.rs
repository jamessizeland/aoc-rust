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

/// A trait to allow parsing directly on strings
/// Usage: "10, -5".nums()
pub trait ParseOps {
    fn nums(&self) -> Vec<isize>;
}

impl ParseOps for str {
    fn nums(&self) -> Vec<isize> {
        extract_numbers(self)
    }
}
