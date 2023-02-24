use std::io::{Stdout, Write};

pub struct Editor {
    x: u16,
    y: u16,
    screen: Stdout,
}

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style::{self, Attribute, Color, Stylize},
    terminal::{self, ClearType},
    Command, Result,
};

impl Editor {
    // 기본값
    pub fn default() -> Editor {
        Editor {
            screen: std::io::stdout(),
            x: 0,
            y: 0,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        // let mut w = &mut self.screen;
        execute!(&self.screen, terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;

        loop {
            match read_char()? {
                'q' => break,
                c => self.put_char(c),
            }
        }

        execute!(&self.screen, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()
    }

    /**
     * 현재 커서 위치에 글자 하나를 출력
     */
    fn put_char(&mut self, ch: char) -> () {
        queue!(&self.screen, cursor::MoveTo(self.x, self.y));
        print!("{}", ch);
        match Write::flush(&mut self.screen) {
            Ok(()) => (),
            Err(error) => {
                panic!("Failed to put char {:?}", error);
            }
        };
        self.x += 1
    }
}

fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = event::read()
        {
            return Ok(c);
        }
    }
}
