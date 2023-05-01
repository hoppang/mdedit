mod menu_group;
mod menu_item;

use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{cursor, queue};
use log::info;
use menu_group::MenuGroup;
use std::convert::TryInto;
use std::io::Stdout;

pub struct MenuBar {
    groups: Vec<MenuGroup>,
}

impl MenuBar {
    pub fn new() -> MenuBar {
        let mut menu_bar = MenuBar { groups: Vec::new() };

        let file_group = MenuGroup::new("File");
        menu_bar.add_group(file_group);

        let help_group = MenuGroup::new("Help");
        menu_bar.add_group(help_group);

        menu_bar
    }

    pub fn add_group(&mut self, new_group: MenuGroup) {
        self.groups.push(new_group);
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

        for (iter, group) in self.groups.iter().enumerate() {
            let x = (iter * 10 + 4).try_into().unwrap();
            queue!(screen, cursor::MoveTo(x, 0)).unwrap();
            print!("{}", group.name);
        }

        queue!(screen, ResetColor).unwrap();
    }
}
