mod menu_group;

use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{cursor, queue};
use log::info;
use menu_group::MenuGroup;
use std::io::Stdout;

pub struct MenuBar {
    groups: Vec<MenuGroup>,
}

impl MenuBar {
    pub fn new() -> MenuBar {
        MenuBar { groups: Vec::new() }
    }

    pub fn draw(&self, mut screen: &Stdout, width: usize) {
        info!("draw menubar: groups = {:?} / {:?}", self.groups, screen);

        queue!(screen, cursor::MoveTo(0, 0)).unwrap();
        queue!(
            screen,
            SetBackgroundColor(Color::DarkCyan),
            SetForegroundColor(Color::White)
        )
        .unwrap();

        for _ in 0..width {
            print!(" ");
        }

        queue!(screen, cursor::MoveTo(4, 0)).unwrap();
        print!("Dummy menubar");
        queue!(screen, ResetColor).unwrap();
    }
}
