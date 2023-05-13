use crate::consts::ui::MenuCmd;

#[derive(Debug)]
pub struct MenuItem {
    pub name: String,
    pub cmd: MenuCmd,
}

impl MenuItem {
    pub fn new(new_name: &str, cmd_val: MenuCmd) -> MenuItem {
        MenuItem {
            name: String::from(new_name),
            cmd: cmd_val,
        }
    }
}
