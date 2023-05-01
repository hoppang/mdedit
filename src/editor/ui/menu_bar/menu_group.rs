use super::menu_item::MenuItem;

#[derive(Debug)]
pub struct MenuGroup {
    pub name: String,
    _items: Vec<MenuItem>,
}

impl MenuGroup {
    pub fn new(group_name: &str) -> MenuGroup {
        MenuGroup {
            name: String::from(group_name),
            _items: Vec::new(),
        }
    }
}
