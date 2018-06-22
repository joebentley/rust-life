use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::{Add, AddAssign};

#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr) => {
        Point::new($x, $y)
    };
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn from_tuple<T : Into<i32>>(tup: (T, T)) -> Point {
        Point { x: tup.0.into(), y: tup.1.into() }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = Point {
            x: self.x + other.x,
            y: self.y + other.y
        };
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
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

    pub fn living_cells(&self) -> &HashSet<Point> {
        &self.living_cells
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

    pub fn flip_cell(&mut self, p: Point) {
        match self.get_cell(p) {
            Cell::Alive => { self.set_cell(p, Cell::Dead) },
            Cell::Dead => { self.set_cell(p, Cell::Alive) }
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

        HashSet::from_iter(neighbour_offsets.iter().map(|&offset| { p + offset }))
    }

    pub fn get_living_neighbours(&self, p: Point) -> HashSet<Point> {
        self.get_neighbours(p).into_iter().filter(|&point| {
            self.get_cell(point) == Cell::Alive
        }).collect()
    }

    /// Return set of points, dead or alive, to be checked for
    /// next generation
    fn to_be_checked(&self) -> HashSet<(Point, Cell)> {
        let mut to_be_checked = HashSet::new();

        // TODO: double loop, slow?
        for &point in self.living_cells.iter() {
            to_be_checked.insert((point, Cell::Alive));
            for neighbour_point in self.get_neighbours(point) {
                to_be_checked.insert((neighbour_point, self.get_cell(neighbour_point)));
            }
        }

        to_be_checked
    }
}

pub struct Automaton {
    board: Board
}

impl Automaton {
    pub fn new() -> Automaton {
        Automaton {
            board: Board::new()
        }
    }

    pub fn set_board(&mut self, board: Board) {
        self.board = board;
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }

    pub fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    pub fn step_next_generation(&mut self) {
        let board = self.get_board_mut();
        let to_be_checked = board.to_be_checked();
        let mut to_be_flipped = HashSet::new();

        for (point, status) in to_be_checked {
            let num_neighbours = board.get_living_neighbours(point).len();

            match status {
                Cell::Alive => {
                    match num_neighbours {
                        2 | 3 => {}, // Continue to next generation
                        _ => {
                            to_be_flipped.insert(point); // Alive -> Dead
                        }
                    }
                },
                Cell::Dead => {
                    if num_neighbours == 3 {
                        to_be_flipped.insert(point); // Dead -> Alive
                    }
                }
            }
        }

        for point in to_be_flipped {
            board.flip_cell(point);
        }
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
    fn get_living_neighbours_should_be_zero_for_no_neighbours() {
        let mut board = Board::new();
        let p = point!(1, 0);
        board.set_cell(p, Cell::Alive);
        assert_eq!(board.get_living_neighbours(p).len(), 0);
    }

    #[test]
    fn to_be_checked_includes_dead_neighbours() {
        let mut board = Board::new();
        board.add_cells(vec![point!(2, 0), point!(3, 0)]);
        let expected: HashSet<Point> = HashSet::from_iter(vec![
            point!(1,  1), point!(2,  1), point!(3,  1), point!(4,  1),
            point!(1,  0), point!(2,  0), point!(3,  0), point!(4,  0),
            point!(1, -1), point!(2, -1), point!(3, -1), point!(4, -1)
        ]);

        let points = HashSet::from_iter(board.to_be_checked().iter().map(|(p, _)| *p));

        assert_eq!(points, expected);
    }

    #[test]
    fn test_blinker() {
        let mut automaton = Automaton::new();
        automaton.get_board_mut().add_cells(vec![point!(1, 2), point!(2, 2), point!(3, 2)]);
        println!("{}", automaton.get_board().living_cells().len());
        automaton.step_next_generation();
        println!("{}", automaton.get_board().living_cells().len());

        let expected = HashSet::from_iter(vec![
            point!(2, 3), point!(2, 2), point!(2, 1)
        ]);

        assert_eq!(automaton.get_board().living_cells(), &expected);
    }

    #[test]
    fn test_lone_cell_should_die_next_generation() {
        let mut automaton = Automaton::new();
        automaton.get_board_mut().set_cell(point!(2, 2), Cell::Alive);
        automaton.step_next_generation();
        assert_eq!(automaton.get_board().get_cell(point!(2, 2)), Cell::Dead);
    }
}