use itertools::Itertools;
use std::{fmt::Display, ops::Add};

/// 2D Grid of data points
pub struct GridInput {
    lines: Vec<String>,
    width: usize,
    height: usize,
    highlighted: Vec<Point>,
}

impl Display for GridInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl GridInput {
    pub(crate) fn new(input: &str) -> Self {
        let lines = input.trim().lines().map(|s| s.to_string()).collect_vec();
        let height = lines.len();
        let Some(first_line) = lines.first() else {
            panic!("grid has no height");
        };
        let width = first_line.chars().count();
        Self {
            lines,
            width,
            height,
            highlighted: Vec::new(),
        }
    }
    /// Mark a data point for highlighting later
    pub fn mark(&mut self, point: Point) {
        self.highlighted.push(point);
    }
    /// Get the height of the puzzle grid
    pub fn height(&self) -> usize {
        self.height
    }
    /// Get the width of the puzzle grid
    pub fn width(&self) -> usize {
        self.width
    }
    /// Iterate through all data points in the puzzle grid
    pub fn iter(&self) -> impl Iterator<Item = char> + '_ {
        self.lines.iter().flat_map(|row| row.chars())
    }
    /// Get the value at x,y in the puzzle grid, if it is a number.
    pub fn get_digit(&self, point: Point) -> Option<u32> {
        self.get(point).and_then(|val| {
            println!("val: {val}");
            val.to_digit(10)
        })
    }
    /// Get the value at x,y in the puzzle grid.
    pub fn get(&self, point: Point) -> Option<char> {
        if point.x > self.width as isize || point.x < 0 {
            return None;
        }
        if point.y > self.height as isize || point.y < 0 {
            return None;
        }
        let Some(line) = self.lines.get(point.y as usize) else {
            return None;
        };
        let mut chars = line.chars();
        chars.nth(point.x as usize)
    }
}

/// Single point within a Grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }

    pub fn vector(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Returns the 4 cardinal neighbors (Up, Down, Left, Right)
    pub fn neighbors(&self) -> Vec<Point> {
        vec![
            Point::vector(self.x, self.y - 1), // Up
            Point::vector(self.x, self.y + 1), // Down
            Point::vector(self.x - 1, self.y), // Left
            Point::vector(self.x + 1, self.y), // Right
        ]
    }

    /// Returns the 8 neighbors (Cardinal + Diagonals)
    pub fn neighbors_extended(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                points.push(Point::vector(self.x + dx, self.y + dy));
            }
        }
        points
    }
}

// Allows generic addition: Point + Point
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Common directional constants
pub const UP: Point = Point { x: 0, y: -1 };
pub const DOWN: Point = Point { x: 0, y: 1 };
pub const LEFT: Point = Point { x: -1, y: 0 };
pub const RIGHT: Point = Point { x: 1, y: 0 };

#[cfg(test)]
mod test {
    use crate::prelude::*;

    const TEST_INPUT: &str = "abcdefg\nhijklmn\nopqrstu\n1234567";

    #[test]
    fn grid_dimensions() {
        let map = Input::from_literal(TEST_INPUT).to_grid();
        assert_eq!(map.width, 7);
        assert_eq!(map.height, 4);
    }

    #[test]
    fn test_inside_grid_point() {
        let map = Input::from_literal(TEST_INPUT).to_grid();
        let point = Point::new(6, 2);
        let result = map.get(point);
        assert_eq!(result, Some('u'))
    }
    #[test]
    fn test_outside_grid_point() {
        let map = Input::from_literal(TEST_INPUT).to_grid();
        let point = Point::new(6, 99);
        let result = map.get(point);
        assert_eq!(result, None)
    }
    #[test]
    fn test_parse_grid_point() {
        let map = Input::from_literal(TEST_INPUT).to_grid();
        let point = Point::new(2, 3);
        let result = map.get_digit(point);
        assert_eq!(result, Some(3))
    }
}
