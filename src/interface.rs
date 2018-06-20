extern crate termion;

use std::io::{stdout, Read, Write};
use std::thread;
use std::time::Duration;

use termion::raw::IntoRawMode;
use termion::async_stdin;

use automaton::{Automaton, Board, Cell, Point};

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

            print!("{}", termion::cursor::Goto(screen_x, screen_y));

            match board.get_cell(world_point) {
                Cell::Alive => { print!("x"); }
                Cell::Dead => { }
            }
        }
    }
}

pub fn run() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    let mut automaton = Automaton::new();
    automaton.get_board_mut().add_cells(vec![point!(1, 2), point!(2, 2), point!(3, 2)]);


    automaton.get_board_mut().add_cells(vec![
        point!(7, 7), point!(8, 7), point!(9, 7), point!(9, 6), point!(8, 5)]);

    let camera = Camera { pos: Point::new(-10, -10), size: Point::from_tuple(termion::terminal_size().unwrap())};

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
}
