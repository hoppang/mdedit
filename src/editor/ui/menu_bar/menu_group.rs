use super::menu_item::MenuItem;
use crate::editor::ui::rect::Rect;
use crossterm::{cursor, queue};

#[derive(Debug)]
pub struct MenuGroup {
    pub name: String,
    index: u16,
    items: Vec<MenuItem>,
}

impl MenuGroup {
    pub fn new(group_name: &str, idx: u16) -> MenuGroup {
        MenuGroup {
            name: String::from(group_name),
            index: idx,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, new_item: MenuItem) {
        self.items.push(new_item);
    }

    pub fn draw(&mut self) {
        let x = self.index * 10 + 2;
        let y = 1;
        let w = 25;
        let h = self.items.len() as u16 + 2;
        Rect::draw(&std::io::stdout(), x, y, w, h);

        for (i, item) in self.items.iter().enumerate() {
            queue!(&std::io::stdout(), cursor::MoveTo(x + 2, y + i as u16 + 1)).unwrap();
            print!("{}", item.name);
        }
    }
}
