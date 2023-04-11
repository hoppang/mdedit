use std::io::{Stdout, Write};

mod line_buffer;
use line_buffer::LineBuffer;

pub struct Editor {
    current_line: u16,
    screen: Stdout,
    line_buffer: LineBuffer,
}

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{self, Attribute, Color, Stylize},
    terminal::{self, ClearType},
    Command, Result,
};

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
                (_, KeyCode::Backspace) => {
                    self.pop_back();
                    self.refresh()
                }
                (_, KeyCode::Char(c)) => {
                    self.put_char(c);
                    self.refresh()
                }
                _ => {} // do nothing
            }
        }

        execute!(&self.screen, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()
    }

    /**
     * 버퍼에 글자 하나 추가
     */
    fn put_char(&mut self, ch: char) {
        self.line_buffer.push(ch)
    }

    /**
     * 버퍼의 마지막 글자 삭제
     */
    fn pop_back(&mut self) {
        self.line_buffer.pop();
    }

    /**
     * 현재 커서가 있는 한 줄 갱신
     */
    fn refresh(&mut self) {
        match queue!(&self.screen, cursor::MoveTo(0, self.current_line)) {
            Ok(()) => (),
            Err(error) => {
                panic!("Failed to queue mouse position: {:?}", error);
            }
        }

        self.line_buffer.draw();

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
