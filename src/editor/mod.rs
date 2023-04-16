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
    contents: Vec<LineBuffer>,
    cursor: Cursor,
}

impl Editor {
    // 기본값
    pub fn new() -> Editor {
        info!("Create new editor object");
        Editor {
            screen: std::io::stdout(),
            cursor: Cursor { x: 0, y: 0 },
            contents: Vec::from([LineBuffer::new()]),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        // let mut w = &mut self.screen;
        execute!(&self.screen, terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        self.refresh(false);

        loop {
            match read_char()? {
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => break,
                (_, KeyCode::F(10)) => break,
                (_, KeyCode::Backspace) => {
                    self.current_line().pop();
                    self.cursor.x = self.current_line().width() as u16;
                    self.refresh(true)
                }
                (_, KeyCode::Char(c)) => {
                    self.current_line().push(c);
                    self.cursor.x = self.current_line().width() as u16;
                    self.refresh(true)
                }
                (KeyModifiers::NONE, KeyCode::Enter) => {
                    self.contents.push(LineBuffer::new());
                    self.cursor.x = 0;
                    self.cursor.y += 1;
                    self.refresh(false)
                }
                (KeyModifiers::NONE, KeyCode::Left) => self.handle_leftkey(true),
                (KeyModifiers::NONE, KeyCode::Right) => self.handle_rightkey(true),
                (KeyModifiers::NONE, KeyCode::Up) => self.handle_upkey(),
                (KeyModifiers::NONE, KeyCode::Down) => self.handle_downkey(),
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
            self.current_line().draw(screen_width());
        }

        // 디버그 정보 출력
        self.print_dbgmsg();

        queue!(&self.screen, cursor::MoveTo(self.cursor.x, self.cursor.y))
            .expect("Failed to move cursor");

        match Write::flush(&mut self.screen) {
            Ok(()) => (),
            Err(error) => {
                panic!("Failed to put char {:?}", error);
            }
        };
    }

    fn print_dbgmsg(&mut self) {
        queue!(&self.screen, cursor::MoveTo(0, screen_height() - 1))
            .expect("Failed to move cursor");

        let x = self.cursor.x;
        let y = self.cursor.y;
        print!(
            "current_line: {:?} cx {:?} cy {}    ",
            self.current_line(),
            x,
            y
        );
    }

    fn current_line(&mut self) -> &mut LineBuffer {
        &mut self.contents[self.cursor.y as usize]
    }

    // ================================================================================
    // 키 입력 핸들러

    fn handle_upkey(&mut self) {
        if self.cursor.y > 0 {
            self.cursor.y -= 1;

            let x = self.calibrate_x();
            self.cursor.x = x;
            self.current_line().set_cursor(x);

            self.refresh(false);
        }

        self.refresh(false);
    }

    fn handle_downkey(&mut self) {
        if self.contents.len() - 1 > self.cursor.y as usize {
            self.cursor.y += 1;

            let x = self.calibrate_x();
            self.cursor.x = x;
            self.current_line().set_cursor(x);

            self.refresh(false);
        }
    }

    /**
        다른 라인으로 넘어갔을 때 x위치 보정
        한글 등 너비가 1이 아닌 문자들이 있을 수도 있기 때문
    */
    fn calibrate_x(&mut self) -> u16 {
        if self.cursor.x > self.current_line().len() as u16 {
            self.cursor.x = self.current_line().len() as u16;
        }

        let x = self.cursor.x;
        let new_x = self.current_line().index_from_width(x);
        new_x
    }

    fn handle_leftkey(&mut self, refresh: bool) {
        self.current_line().prev();
        let char_width = self.current_line().current_char_width() as u16;
        self.cursor.move_left(char_width);

        if refresh {
            self.refresh(false)
        }
    }

    fn handle_rightkey(&mut self, refresh: bool) {
        let char_width = self.current_line().current_char_width() as u16;
        self.cursor.move_right(char_width);
        let _no_use = self.current_line().next();

        if refresh {
            self.refresh(false)
        }
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

fn screen_height() -> u16 {
    match size() {
        Ok((_cols, rows)) => rows,
        Err(error) => panic!("screen_width: {:?}", error),
    }
}
