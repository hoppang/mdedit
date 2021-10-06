use std::io::{self, Write};
pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command, Result,
};

fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {code: KeyCode::Char(c), ..})) = event::read() {
            return Ok(c);
        }
    }
}

fn run<W>(w: &mut W) -> Result<()> where W: Write {
    execute!(w, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    match read_char()? {
        _ => {}
    }

    execute!(w, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()
}

fn main() -> Result<()> {
    let mut stdout = io::stdout();
    run(&mut stdout)
}
