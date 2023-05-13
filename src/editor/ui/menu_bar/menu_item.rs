use crate::consts::ui::MenuCmd;
use crate::Editor;

#[derive(Debug)]
pub struct MenuItem {
    pub name: String,
    pub job: MenuCmd,
}

impl MenuItem {
    pub fn new(new_name: &str, job_no: MenuCmd) -> MenuItem {
        MenuItem {
            name: String::from(new_name),
            job: job_no,
        }
    }

    pub fn invoke(&self, editor: &Editor) {
        editor.invoke_menu(self.job);
    }
}
