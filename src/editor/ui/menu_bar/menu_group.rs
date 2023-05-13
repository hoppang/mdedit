use super::menu_item::MenuItem;
use crate::consts::ui;
use crate::editor::ui::rect::Rect;
use crate::editor::util::set_color;
use crossterm::{cursor, queue};

#[derive(Debug)]
pub struct MenuGroup {
    pub name: String,
    index: u16,
    items: Vec<MenuItem>,
    selected: usize,
}

impl MenuGroup {
    pub fn new(group_name: &str, idx: u16) -> MenuGroup {
        MenuGroup {
            name: String::from(group_name),
            index: idx,
            items: Vec::new(),
            selected: 0,
        }
    }

    pub fn get_menu_cmd(&self) -> ui::MenuCmd {
        self.items[self.selected].cmd
    }

    pub fn add_item(&mut self, new_item: MenuItem) {
        self.items.push(new_item);
    }

    pub fn draw(&mut self) {
        set_color(ui::MENU_COLOR, ui::MENU_BGCOLOR);

        let x = self.index * 10 + 2;
        let y = 1;
        let w = 25;
        let h = self.items.len() as u16 + 2;
        Rect::draw(&std::io::stdout(), x, y, w, h);

        for (i, item) in self.items.iter().enumerate() {
            queue!(&std::io::stdout(), cursor::MoveTo(x + 2, y + i as u16 + 1)).unwrap();
            if self.selected == i {
                set_color(ui::MENU_COLOR, ui::MENU_BGCOLOR_SELECTED);
            } else {
                set_color(ui::MENU_COLOR, ui::MENU_BGCOLOR);
            }
            print!("{}", item.name);
        }
    }
}
