use std::io::{Read};
use regex::Regex;
use automaton::{Board, Point, Cell};

pub struct RleLoader {
    pub width: u32,
    pub height: u32,
    pub board: Board
}

impl RleLoader {
    pub fn from_string(string: &str) -> RleLoader {
        RleLoader::from_reader(string.as_bytes())
    }

    pub fn from_reader<T : Read>(mut reader: T) -> RleLoader {
        let mut string = String::new();
        reader.read_to_string(&mut string).unwrap();

        let re = Regex::new(r"x *= *(\d+), *y *= *(\d+)").unwrap();

        let captures = re.captures(&string).unwrap();

        let width: u32 = captures.get(1).unwrap().as_str().parse().unwrap();
        let height: u32 = captures.get(2).unwrap().as_str().parse().unwrap();

        let end_of_match_index = captures.get(2).unwrap().end();

        let string = string.get(end_of_match_index..).unwrap();
        let string = string.split("\n").nth(1).unwrap(); // Get rest of data

        let re = Regex::new(r"([\s\S]*)!").unwrap();
        let data = re.captures(string).unwrap().get(1).unwrap().as_str().replace(r"\n", "");

        let mut p = Point::new(0, 0);
        let mut board = Board::new();

        for line in data.split("$") {
            let re = Regex::new(r"(\d*(?:b|o){1})").unwrap();
            let matches = re.captures_iter(line);
            for capture in matches {
                let instruction = capture.get(0).unwrap().as_str();

                let re = Regex::new(r"(\d)*(\w)").unwrap();

                let count = match re.captures(instruction).unwrap().get(1) {
                    None => 1,
                    Some(m) => m.as_str().parse().unwrap()
                };

                let status = match re.captures(instruction).unwrap().get(2).unwrap().as_str() {
                    "b" => Cell::Dead,
                    "o" => Cell::Alive,
                    _ => panic!("Bad cell type!")
                };

                for _ in 0..count {
                    board.set_cell(p, status);
                    p += point!(1, 0);
                }
            }

            p.x = 0;
            p += point!(0, 1);
        }

        RleLoader { width, height, board }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_simple_file() {
        let string = "x = 3, y = 4\nbo$2bo$3o!";
        let loader = RleLoader::from_string(string);
        assert_eq!(loader.width, 3);
        assert_eq!(loader.height, 4);
        let board = loader.board;
        assert_eq!(board.num_living(), 5);
        assert_eq!(board.get_cell(point!(0, 2)), Cell::Alive);
        assert_eq!(board.get_cell(point!(1, 2)), Cell::Alive);
        assert_eq!(board.get_cell(point!(2, 2)), Cell::Alive);
        assert_eq!(board.get_cell(point!(1, 0)), Cell::Alive);
        assert_eq!(board.get_cell(point!(2, 1)), Cell::Alive);
    }

    #[test]
    fn test_parsing_gosper_gun() {
        let string = "x = 36, y = 9
24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4b
obo$10bo5bo7bo$11bo3bo$12b2o!";
        let loader = RleLoader::from_string(string);
        assert_eq!(loader.board.num_living(), 36);
    }
}