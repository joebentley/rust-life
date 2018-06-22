extern crate termion;

use std::io::{stdout, Read, Write};
use std::thread;
use std::time::Duration;
use std::fs::File;

use termion::raw::IntoRawMode;
use termion::async_stdin;

use automaton::{Automaton, Board, Cell, Point};
use loader::RleLoader;

struct Camera {
    pos: Point,
    size: Point
}

fn draw_board(board: &Board, camera: &Camera) {
    print!("{}", termion::clear::All);

    for y in 0 .. camera.size.y - 1 {
        for x in 0 .. camera.size.x - 1 {
            let world_point = camera.pos + Point::new(x, y);
            let (screen_x, screen_y): (u16, u16) = ((x + 1) as u16, (y + 1) as u16);

            match board.get_cell(world_point) {
                Cell::Alive => {
                    print!("{}", termion::cursor::Goto(screen_x, screen_y));
                    print!("x");
                }
                Cell::Dead => { }
            }
        }
    }
}

fn example_board() -> Board {
    let string = "x = 3, y = 4\nbo$2bo$3o!";
    RleLoader::from_string(string).board
}

fn load_board_from_file(filepath: &str) -> Board {
    let f = File::open(filepath).unwrap();
    RleLoader::from_reader(f).board
}

pub fn run(filepath: Option<&str>) {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let mut automaton = Automaton::new();

    automaton.set_board(match filepath {
        Some(path) => load_board_from_file(&path),
        None => example_board()
    });

    let camera = Camera { pos: Point::new(-10, -10), size: Point::from_tuple(termion::terminal_size().unwrap())};

    print!("{}", termion::cursor::Hide);

    loop {
        draw_board(automaton.get_board(), &camera);

        let b = stdin.next();

        if let Some(Ok(b'q')) = b {
            break;
        }

        stdout.flush().unwrap();
        automaton.step_next_generation();
        thread::sleep(Duration::from_millis(100));
    }

    print!("{}", termion::cursor::Show);
}
