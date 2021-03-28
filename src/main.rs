extern crate ncurses;

use ncurses::*;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut contents = String::from("");
    if args.len() == 2 {
        let filename = args[1].as_str();
        contents = fs::read_to_string(filename).expect("error");
    }

    initscr();
    addstr(contents.as_str());
    refresh();
    getch();
    endwin();
}
