use crossterm::terminal::size;
use std::io::{Stdout, Write};

mod line_buffer;
use line_buffer::LineBuffer;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{self, Attribute, Color, Stylize},
    terminal::{self, ClearType},
    Command, Result,
};

pub struct Editor {
    current_line: u16,
    screen: Stdout,
    line_buffer: LineBuffer,
}

impl Editor {
    // 기본값
    pub fn default() -> Editor {
        Editor {
            screen: std::io::stdout(),
            current_line: 0,
            line_buffer: LineBuffer::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        // let mut w = &mut self.screen;
        execute!(&self.screen, terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;

        loop {
            match read_char()? {
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => break,
                (_, KeyCode::F(10)) => break,
                (_, KeyCode::Backspace) => {
                    self.line_buffer.pop();
                    self.refresh()
                }
                (_, KeyCode::Char(c)) => {
                    self.line_buffer.push(c);
                    self.refresh()
                }
                _ => {} // do nothing
            }
        }

        execute!(&self.screen, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()
    }

    /**
     * 현재 커서가 있는 한 줄 갱신
     */
    fn refresh(&mut self) {
        queue!(&self.screen, cursor::MoveTo(0, self.current_line)).expect("Failed to move cursor");

        self.line_buffer.draw(screen_width());
        queue!(
            &self.screen,
            cursor::MoveTo(self.line_buffer.width() as u16, self.current_line)
        )
        .expect("Failed to move cursor");

        match Write::flush(&mut self.screen) {
            Ok(()) => (),
            Err(error) => {
                panic!("Failed to put char {:?}", error);
            }
        };
    }
}

fn read_char() -> Result<(KeyModifiers, KeyCode)> {
    loop {
        // rust 의 char 크기는 4바이트이므로 한글도 들어감.
        if let Ok(Event::Key(KeyEvent {
            code: c,
            modifiers: m,
        })) = event::read()
        {
            return Ok((m, c));
        }
    }
}

fn screen_width() -> usize {
    match size() {
        Ok((cols, _rows)) => cols as usize,
        Err(error) => panic!("screen_width: {:?}", error),
    }
}
