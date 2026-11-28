use crate::helpers::point::Point;
use colored::Colorize;
use std::{cell::RefCell, collections::HashSet, fmt::Display};

/// 2D Grid of data points
pub struct GridInput {
    cells: Vec<Vec<char>>,
    width: usize,
    height: usize,
    highlighted: RefCell<HashSet<Point>>,
}

impl Display for GridInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (point, value) in self.iter() {
            if point.x == 0 && point.y > 0 {
                writeln!(f)?; // add newline
            }
            match self.is_marked(point) {
                true => write!(f, "{}", value.to_string().red())?,
                false => write!(f, "{}", value)?,
            }
        }
        Ok(())
    }
}

impl GridInput {
    pub(crate) fn new(input: &str) -> Self {
        let cells: Vec<Vec<char>> = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let height = cells.len();
        let width = cells.first().map_or(0, Vec::len);

        assert!(height > 0, "grid has no height");
        debug_assert!(
            cells.iter().all(|row| row.len() == width),
            "grid rows have inconsistent widths"
        );

        Self {
            cells,
            width,
            height,
            highlighted: RefCell::new(HashSet::new()),
        }
    }
    /// Mark a data point for highlighting later
    pub fn mark(&self, point: Point) {
        self.highlighted.borrow_mut().insert(point);
    }
    /// Check if a data point is marked as highlighted
    pub fn is_marked(&self, point: Point) -> bool {
        self.highlighted.borrow().contains(&point)
    }
    /// Get the height of the puzzle grid
    pub fn height(&self) -> usize {
        self.height
    }
    /// Get the width of the puzzle grid
    pub fn width(&self) -> usize {
        self.width
    }
    /// Get the value at x,y in the puzzle grid, if it is a number.
    pub fn get_digit(&self, point: Point) -> Option<u32> {
        self.get(point).and_then(|val| val.to_digit(10))
    }
    /// Returns an iterator over the slice.
    /// The iterator yields all items from start to end.
    pub fn iter(&self) -> impl Iterator<Item = (Point, char)> + '_ {
        self.cells.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &ch)| (Point::new(x, y), ch))
        })
    }
    /// Iterate through all of the orthogonal neighbouring coordinates of
    /// this point in the puzzle grid.
    pub fn neighbours(&self, point: Point) -> impl Iterator<Item = (Point, char)> + '_ {
        point
            .neighbours()
            .into_iter()
            .filter_map(|p| self.get(p).map(|ch| (p, ch)))
    }
    /// Iterate through all of the orthogonal and diagonal neighbouring
    /// coordinates of this point in the puzzle grid.
    pub fn neighbours_extended(&self, point: Point) -> impl Iterator<Item = (Point, char)> + '_ {
        point
            .neighbours_extended()
            .into_iter()
            .filter_map(|p| self.get(p).map(|ch| (p, ch)))
    }
    /// Get the value at x,y in the puzzle grid.
    pub fn get(&self, point: Point) -> Option<char> {
        let x = usize::try_from(point.x).ok()?;
        let y = usize::try_from(point.y).ok()?;
        self.cells.get(y)?.get(x).copied()
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    const TEST_INPUT: &str = "abcdefg\nhijklmn\nopqrstu\n1234567";

    fn get_test_input() -> GridInput {
        Input::from_literal(TEST_INPUT).to_grid()
    }

    #[test]
    fn grid_dimensions() {
        let map = get_test_input();
        assert_eq!(map.width, 7);
        assert_eq!(map.height, 4);
    }

    #[test]
    fn test_inside_grid_point() {
        let map = get_test_input();
        let point = Point::new(6, 2);
        let result = map.get(point);
        assert_eq!(result, Some('u'))
    }
    #[test]
    fn test_outside_grid_point() {
        let map = get_test_input();
        let point = Point::new(6, 99);
        let result = map.get(point);
        assert_eq!(result, None)
    }
    #[test]
    fn test_parse_grid_point() {
        let map = get_test_input();
        let point = Point::new(2, 3);
        let result = map.get_digit(point);
        assert_eq!(result, Some(3))
    }
    #[test]
    fn iterate_over_grid_points() {
        let map = get_test_input();
        for (point, _value) in map.iter() {
            // coordinates should run from (0,0) to (W,H) row by row
            if map
                .neighbours(point)
                .map(|(_point, value)| value)
                .contains(&'j')
            {
                map.mark(point);
            }
        }
        println!("{map}"); // show map with marked datapoints
        assert!(map.is_marked(Point::new(2, 0)))
    }
}
