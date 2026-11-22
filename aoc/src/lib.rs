use itertools::Itertools as _;

mod helpers {
    pub mod file;
    pub mod grid;
    pub mod parsing;
}

pub mod prelude {
    #![allow(unused)]

    use super::helpers;
    pub use anyhow::{Result, anyhow};
    pub use colored::*;
    pub use helpers::file::*;
    pub use helpers::grid::*;
    pub use helpers::parsing::*;
    pub use itertools::Itertools;
    pub use rayon::prelude::*;
}

pub fn get_puzzle_input() -> String {
    let exe_path = std::env::current_exe().expect("Failed to get current executable path");
    let bin_name = exe_path.file_stem().expect("not a file").to_str().unwrap();
    let (year, day) = bin_name
        .split("_day")
        .collect_tuple()
        .expect("puzzle input name should be YYYY_dayDD");
    let input_path = format!("puzzles/input/{year}/day{day}/input.aoc");
    std::fs::read_to_string(&input_path)
        .expect("file should be a string")
        .trim()
        .to_string()
}
