#[derive(Debug)]
pub struct MenuItem {
    _name: String,
}

impl MenuItem {
    pub fn new(new_name: &str) -> MenuItem {
        MenuItem {
            _name: String::from(new_name),
        }
    }
}
