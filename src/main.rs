extern crate termion;

fn main() {
    print!("{}{}hello", termion::clear::All, termion::cursor::Goto(5, 3));
}
