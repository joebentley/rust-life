use std::io::{Read};
use automaton::{Board, Point, Cell};
use nom;
use std::str;

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

        named!(skip_comment<&str, &str>,
            do_parse!(ws!(preceded!(opt!(call!(nom::space)), tag!("#"))) >>
                      many_till!(call!(nom::anychar), nom::line_ending) >> ("")));

        named!(skip_comments<&str, &str>,
            do_parse!(many0!(skip_comment) >> ("")));

        let a = skip_comments(string.as_str());

        named!(parse_number<&str, u32>,
            map_res!(call!(nom::digit), |string: &str| string.parse() ));

        named!(parse_width_and_height<&str, (u32, u32)>,
            complete!(
            do_parse!(ws!(tag!("x =")) >> x: parse_number >> tag!(",") >>
                      ws!(tag!("y =")) >> y: parse_number >> many_till!(call!(nom::anychar), nom::line_ending) >> (x, y))));

        let a = parse_width_and_height(a.unwrap().0);
        let (rest, (width, height)) = a.unwrap();

        #[derive(Debug)]
        struct CountAndStatus {
            count: u32,
            status: Cell
        }

        named!(parse_cell<&str, CountAndStatus>,
            do_parse!(
            count: opt!(call!(nom::digit)) >>
            status: alt!(tag!("b") | tag!("o")) >>
            (CountAndStatus {
                count: count.unwrap_or("1").parse().unwrap(),
                status: if status == "b" { Cell::Dead } else { Cell::Alive }
            })
        ));

        let data = rest.replace("\r\n", "");
        let data = data.replace("\n", "");
        let data = data.split("$");

        let mut p = point!(0, 0);
        let mut board = Board::new();

        for line in data {
            let mut remaining = line;

            while let Ok((rem, count_and_status)) = parse_cell(remaining) {
                for _ in 0..count_and_status.count {
                    board.set_cell(p, count_and_status.status); // only need to do this for living cells really
                    p += point!(1, 0);
                }

                remaining = rem;
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
        let string = "
#N Gosper glider gun
#O Bill Gosper
#C A true period 30 glider gun.
#C The first known gun and the first known finite pattern with unbounded growth.
#C www.conwaylife.com/wiki/index.php?title=Gosper_glider_gun
x = 36, y = 9, rule = B3/S23
24bo11b$22bobo11b$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o14b$2o8b
o3bob2o4bobo11b$10bo5bo7bo11b$11bo3bo20b$12b2o!";
        let loader = RleLoader::from_string(string);
        assert_eq!(loader.board.num_living(), 36);
    }
}