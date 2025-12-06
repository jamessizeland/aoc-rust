use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Returns the 4 cardinal neighbors (Up, Down, Left, Right)
    pub fn neighbors(&self) -> Vec<Point> {
        vec![
            Point::new(self.x, self.y - 1), // Up
            Point::new(self.x, self.y + 1), // Down
            Point::new(self.x - 1, self.y), // Left
            Point::new(self.x + 1, self.y), // Right
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
                points.push(Point::new(self.x + dx, self.y + dy));
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
