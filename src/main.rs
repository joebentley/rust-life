extern crate rust_life;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    rust_life::interface::run(args.get(1).map(|s| s.as_str()));
}
