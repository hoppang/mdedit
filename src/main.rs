/**
 * @author Bohun Kim
 */
mod editor;

use editor::Editor;

use crossterm::terminal::size;
use log::{info, LevelFilter};

fn setup_log() {
    simple_logging::log_to_file("dev.log", LevelFilter::Info).unwrap()
}

fn main() -> Result<(), std::io::Error> {
    setup_log();
    let (cols, rows) = size()?;
    info!("cols = {}, rows = {}", cols, rows);
    let mut ed = Editor::default();
    ed.run()
}
