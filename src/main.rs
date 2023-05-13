#![deny(warnings)]

/**
 * @author Bohun Kim
 */
mod editor;
mod consts {
    pub mod ui;
}

use editor::Editor;

use crossterm::terminal;
use crossterm::terminal::size;
use log::{info, LevelFilter};

fn setup_log() {
    simple_logging::log_to_file("dev.log", LevelFilter::Info).unwrap()
}

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode")
    }
}

fn main() -> Result<(), std::io::Error> {
    setup_log();
    let (cols, rows) = size()?;
    info!("cols = {}, rows = {}", cols, rows);
    let mut ed = Editor::new();
    ed.run()
}
