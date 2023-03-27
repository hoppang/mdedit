/**
 * @author Bohun Kim
 */
mod editor;
use editor::Editor;
use log::{info, LevelFilter};
use crossterm::terminal::size;

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
