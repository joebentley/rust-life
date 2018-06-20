
use std::collections::HashSet;

#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr) => {
        Point::new($x, $y)
    };
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Cell {
    Dead,
    Alive,
}

#[derive(Debug)]
pub struct Board {
    living_cells: HashSet<Point>
}

impl Board {
    pub fn new() -> Board {
        Board {
            living_cells: HashSet::new()
        }
    }

    pub fn get_cell(&self, p: Point) -> Cell {
        match self.living_cells.get(&p) {
            Some(_) => Cell::Alive,
            None => Cell::Dead
        }
    }

    pub fn set_cell(&mut self, p: Point, cell: Cell) {
        match cell {
            Cell::Alive => { self.living_cells.insert(p); }
            Cell::Dead => { self.living_cells.remove(&p); }
        }
    }

    pub fn add_cells(&mut self, ps: Vec<Point>) {
        for p in ps {
            self.set_cell(p, Cell::Alive);
        }
    }

    pub fn num_living(&self) -> usize {
        self.living_cells.len()
    }

    pub fn get_neighbours(&self, p: Point) -> HashSet<Point> {
        let neighbour_offsets = vec![
            Point::new(-1, -1), Point::new( 0, -1), Point::new( 1, -1),
            Point::new(-1,  0),                     Point::new( 1,  0),
            Point::new(-1,  1), Point::new( 0,  1), Point::new( 1,  1)
        ];

        let mut set = HashSet::new();

        for offset in neighbour_offsets {
            set.insert(p + offset);
        }

        set
    }

    pub fn get_living_neighbours(&self, p: Point) -> HashSet<Point> {
        self.get_neighbours(p).into_iter().filter(|&point| {
            self.get_cell(point) == Cell::Alive
        }).collect()
    }

    /// Return set of points, dead or alive, to be checked for
    /// next generation
    fn to_be_checked(&self) -> HashSet<Point> {
        let mut to_be_checked = HashSet::new();

        // TODO: double loop, slow?
        for &point in self.living_cells.iter() {
            for neighbour_point in self.get_neighbours(point) {
                to_be_checked.insert(neighbour_point);
            }
        }

        to_be_checked
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn board_should_initially_have_no_living_cells() {
        let board = Board::new();

        assert_eq!(board.num_living(), 0);
    }

    #[test]
    fn setting_cell() {
        let mut board = Board::new();
        let p = Point { x: 1, y: 1 };
        assert_eq!(board.get_cell(p), Cell::Dead);
        board.set_cell(p, Cell::Alive);
        assert_eq!(board.get_cell(p), Cell::Alive);
    }

    #[test]
    fn get_neighbours() {
        let board = Board::new();
        let p = point!(1, 3);
        let neighbours = board.get_neighbours(p);
        assert_eq!(neighbours.len(), 8);
        assert!(neighbours.get(&point!(1, 4)).is_some());
    }

    #[test]
    fn get_living_neighbours() {
        let mut board = Board::new();
        let p = point!(1, 3);
        let living = HashSet::from_iter(vec![point!(0, 4), point!(2, 2)]);

        for point in living.iter() {
            board.set_cell(*point, Cell::Alive);
        }

        assert_eq!(board.get_living_neighbours(p), living);
    }

    #[test]
    fn to_be_checked_includes_dead_neighbours() {
        let mut board = Board::new();
        board.add_cells(vec![point!(2, 0), point!(3, 0)]);
        let expected = HashSet::from_iter(vec![
            point!(1,  1), point!(2,  1), point!(3,  1), point!(4,  1),
            point!(1,  0), point!(2,  0), point!(3,  0), point!(4,  0),
            point!(1, -1), point!(2, -1), point!(3, -1), point!(4, -1)
        ]);

        assert_eq!(board.to_be_checked(), expected);
    }
}