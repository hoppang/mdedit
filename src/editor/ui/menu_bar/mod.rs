mod menu_group;
mod menu_item;

use crate::consts::ui;
use crate::consts::ui::MenuCmd;
use crate::editor::util::set_color;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::style::ResetColor;
use crossterm::{cursor, queue};
use log::info;
use menu_group::MenuGroup;
use menu_item::MenuItem;
use std::convert::TryInto;
use std::io::Stdout;

pub struct MenuBar {
    groups: Vec<MenuGroup>,
    pub selected: Option<usize>,
}

impl MenuBar {
    pub fn new() -> MenuBar {
        let mut menu_bar = MenuBar {
            groups: Vec::new(),
            selected: None,
        };

        let mut file_group = MenuGroup::new("File", 0);
        let exit_item = MenuItem::new("Exit", MenuCmd::Exit);
        file_group.add_item(exit_item);
        menu_bar.add_group(file_group);

        let mut help_group = MenuGroup::new("Help", 1);
        let about_item = MenuItem::new("About", MenuCmd::About);
        help_group.add_item(about_item);
        menu_bar.add_group(help_group);

        menu_bar
    }

    pub fn add_group(&mut self, new_group: MenuGroup) {
        self.groups.push(new_group);
    }

    pub fn handle_keyinput(&mut self, modifier: KeyModifiers, code: KeyCode) -> MenuCmd {
        match (modifier, code) {
            (KeyModifiers::NONE, KeyCode::Enter) => match self.selected {
                Some(idx) => self.groups[idx].get_menu_cmd(),
                _ => MenuCmd::None,
            },
            (KeyModifiers::NONE, KeyCode::Left) => self.move_left(),
            (KeyModifiers::NONE, KeyCode::Right) => self.move_right(),
            (KeyModifiers::NONE, KeyCode::Esc) => MenuCmd::CloseMenu,
            _ => MenuCmd::None,
        }
    }

    pub fn draw(&mut self, mut screen: &Stdout, width: usize) {
        info!("draw menubar: groups = {:?} / {:?}", self.groups, screen);

        self.draw_empty_background(screen, width);

        for (iter, group) in self.groups.iter().enumerate() {
            self.draw_name(screen, iter, &group.name);
        }

        match self.selected {
            Some(idx) => {
                info!("some selected: {}", idx);
                set_color(ui::MENU_COLOR, ui::MENU_BGCOLOR_SELECTED);
                self.draw_name(screen, idx, &self.groups[idx].name);
                self.groups[idx].draw();
            }
            None => info!("Not selected"),
        }

        queue!(screen, ResetColor).unwrap();
    }

    fn draw_empty_background(&self, mut screen: &Stdout, width: usize) {
        queue!(screen, cursor::MoveTo(0, 0)).unwrap();
        set_color(ui::MENU_COLOR, ui::MENU_BGCOLOR);

        for _ in 0..width {
            print!(" ");
        }
    }

    fn draw_name(&self, mut screen: &Stdout, idx: usize, name: &String) {
        let x = (idx * 10 + 4).try_into().unwrap();
        queue!(screen, cursor::MoveTo(x, 0)).unwrap();
        print!("{}", name);
    }

    fn move_left(&mut self) -> MenuCmd {
        info!("left - selected = {:?}", self.selected);
        if let Some(idx) = self.selected {
            let new_idx = if idx == 0 {
                self.groups.len() - 1
            } else {
                idx - 1
            };

            self.selected = Some(new_idx)
        }
        info!("left - selected 2 = {:?}", self.selected);

        MenuCmd::Refresh
    }

    fn move_right(&mut self) -> MenuCmd {
        if let Some(idx) = self.selected {
            let new_idx = if idx == (self.groups.len() - 1) {
                0
            } else {
                idx + 1
            };
            self.selected = Some(new_idx)
        }

        MenuCmd::Refresh
    }
}
