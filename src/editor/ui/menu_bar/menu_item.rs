#[derive(Debug)]
pub struct MenuItem {
    pub name: String,
}

impl MenuItem {
    pub fn new(new_name: &str) -> MenuItem {
        MenuItem {
            name: String::from(new_name),
        }
    }
}
