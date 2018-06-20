extern crate termion;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    print!(
        "{}{}hello",
        termion::clear::All,
        termion::cursor::Goto(5, 3)
    );
    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();

        match evt {
            Event::Key(Key::Char('q')) => break,
            _ => {}
        }
        stdout.flush().unwrap();
    }
}
