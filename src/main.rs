/**
 * @author Bohun Kim
 */
mod editor;
use editor::Editor;

fn main() -> Result<(), std::io::Error> {
    let mut ed = Editor::default();
    ed.run()
}
