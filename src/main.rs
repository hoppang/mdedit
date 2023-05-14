#![deny(warnings)]

/**
 * @author Bohun Kim
 */
mod editor;
mod consts {
    pub mod ui;
}
mod macros;

use editor::Editor;

use crossterm::terminal;
use crossterm::terminal::size;
use log::{info, LevelFilter};

fn setup_log() {
    match simple_logging::log_to_file("dev.log", LevelFilter::Info) {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to start log: {:?}", e);
            std::process::exit(1);
        }
    }
}

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        check_result!(terminal::disable_raw_mode(), "Unable to disable raw mode");
    }
}

fn main() -> Result<(), std::io::Error> {
    setup_log();
    let (cols, rows) = size()?;
    info!("cols = {}, rows = {}", cols, rows);
    let mut ed = Editor::new();
    ed.run()
}
