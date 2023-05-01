use super::menu_item::MenuItem;

#[derive(Debug)]
pub struct MenuGroup {
    pub name: String,
    items: Vec<MenuItem>,
}

impl MenuGroup {
    pub fn new(group_name: &str) -> MenuGroup {
        MenuGroup {
            name: String::from(group_name),
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, new_item: MenuItem) {
        self.items.push(new_item);
    }
}
