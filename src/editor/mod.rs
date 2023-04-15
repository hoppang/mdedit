use crossterm::terminal::size;
use log::info;
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

struct Cursor {
    x: u16,
    y: u16,
}

impl Cursor {
    fn move_left(&mut self, x: u16) {
        if x <= self.x {
            self.x -= x;
        }
    }

    fn move_right(&mut self, x: u16) {
        if self.x + x <= screen_width() as u16 {
            self.x += x;
        }
    }
}

pub struct Editor {
    screen: Stdout,
    line_buffer: LineBuffer,
    cursor: Cursor,
}

impl Editor {
    // 기본값
    pub fn default() -> Editor {
        info!("Create new editor object");
        Editor {
            screen: std::io::stdout(),
            cursor: Cursor { x: 0, y: 0 },
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
                    self.cursor.x = self.line_buffer.width() as u16;
                    self.refresh(true)
                }
                (_, KeyCode::Char(c)) => {
                    self.line_buffer.push(c);
                    self.cursor.x = self.line_buffer.width() as u16;
                    self.refresh(true)
                }
                (KeyModifiers::NONE, KeyCode::Left) => {
                    self.line_buffer.prev();
                    self.cursor
                        .move_left(self.line_buffer.current_char_width() as u16);
                    self.refresh(false)
                }
                (KeyModifiers::NONE, KeyCode::Right) => {
                    self.cursor
                        .move_right(self.line_buffer.current_char_width() as u16);
                    let _no_use = self.line_buffer.next();
                    self.refresh(false)
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
    fn refresh(&mut self, draw_line: bool) {
        queue!(&self.screen, cursor::MoveTo(0, self.cursor.y)).expect("Failed to move cursor");

        if draw_line {
            self.line_buffer.draw(screen_width());
        }
        queue!(&self.screen, cursor::MoveTo(self.cursor.x, self.cursor.y))
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
