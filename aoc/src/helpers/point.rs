use std::ops::Add;

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
    pub fn neighbours(&self) -> Vec<Point> {
        vec![
            Point::vector(self.x, self.y - 1), // Up
            Point::vector(self.x, self.y + 1), // Down
            Point::vector(self.x - 1, self.y), // Left
            Point::vector(self.x + 1, self.y), // Right
        ]
    }

    /// Returns the 8 neighbors (Cardinal + Diagonals)
    pub fn neighbours_extended(&self) -> Vec<Point> {
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
