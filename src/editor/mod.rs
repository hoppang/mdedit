use crossterm::terminal::size;
use log::info;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Stdout, Write};
use unicode_width::UnicodeWidthChar;

mod line_buffer;
use line_buffer::LineBuffer;

mod simple_dialog;
use simple_dialog::SimpleDialog;

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

enum RefreshOption {
    None,
    Line,
    Screen,
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

        let mut ed = Editor {
            screen: std::io::stdout(),
            cursor: Cursor { x: 0, y: 0 },
            contents: Vec::from([LineBuffer::new()]),
        };

        let args: Vec<String> = env::args().collect();
        if args.len() == 2 {
            info!("Open file {:?}", args[1]);
            ed.open_file(&args[1])
        }

        ed
    }

    pub fn run(&mut self) -> Result<()> {
        execute!(&self.screen, terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        self.refresh(RefreshOption::Screen);

        loop {
            match read_char()? {
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => break,
                (KeyModifiers::CONTROL, KeyCode::Char('s')) => self.handle_save(),
                (KeyModifiers::NONE, KeyCode::F(1)) => self.handle_help(),
                (_, KeyCode::F(10)) => break,
                (_, KeyCode::Backspace) => self.handle_backspace(),
                (KeyModifiers::NONE, KeyCode::Char(c)) => self.handle_input_char(c),
                (KeyModifiers::NONE, KeyCode::Enter) => self.handle_enterkey(),
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

    fn open_file(&mut self, filename: &String) {
        self.contents.clear();

        let file = File::open(filename).unwrap();
        for line in io::BufReader::new(file).lines() {
            info!("line = {:?}", line);
            // let text = line.unwrap();
            self.contents.push(LineBuffer::from(&line.unwrap()));
        }
    }

    /**
     * 현재 커서가 있는 한 줄 갱신
     */
    fn refresh(&mut self, opt: RefreshOption) {
        queue!(&self.screen, cursor::MoveTo(0, self.cursor.y)).expect("Failed to move cursor");

        let mut line_count = 0;

        match opt {
            RefreshOption::Line => self.current_line().draw(screen_width()),
            RefreshOption::Screen => {
                queue!(&self.screen, Clear(ClearType::All), cursor::MoveTo(0, 0))
                    .expect("Failed to move cursor");

                for line in &self.contents {
                    info!(
                        "화면에 그리기: x {} y {} line {:?}",
                        line_count, self.cursor.y, line
                    );
                    queue!(&self.screen, cursor::MoveTo(0, line_count))
                        .expect("Failed to move cursor");
                    line.draw(screen_width());
                    line_count += 1;
                }

                queue!(&self.screen, cursor::MoveTo(self.cursor.x, self.cursor.y))
                    .expect("Failed to move cursor");
            }
            _ => {}
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

    fn add_new_line(&mut self) {
        self.contents.push(LineBuffer::new());
        self.cursor.x = 0;
        self.cursor.y += 1;
    }

    fn move_up(&mut self) {
        if self.cursor.y > 0 {
            self.cursor.y -= 1;

            let x = self.cursor.x as i32;
            let (new_x, new_byte_index) = self.current_line().cursor_and_byteindex(x);
            self.cursor.x = new_x;
            self.current_line().set_byte_index(new_byte_index);
        }
    }

    fn move_down(&mut self) {
        if self.contents.len() - 1 > self.cursor.y as usize {
            self.cursor.y += 1;

            let x = self.cursor.x as i32;
            let (new_x, new_byte_index) = self.current_line().cursor_and_byteindex(x);
            self.cursor.x = new_x;
            self.current_line().set_byte_index(new_byte_index);
        }
    }

    // ================================================================================
    // 키 입력 핸들러

    fn handle_help(&mut self) {
        let dialog = SimpleDialog::new();
        dialog.draw(String::from(
            "mdedit: simple text editor inspired by MS-DOS EDIT",
        ));
        self.refresh(RefreshOption::None);
    }

    fn handle_input_char(&mut self, ch: char) {
        self.current_line().insert(ch);
        self.cursor.move_right(ch.width_cjk().unwrap() as u16);
        self.refresh(RefreshOption::Line);
    }

    fn handle_enterkey(&mut self) {
        self.add_new_line();
        self.refresh(RefreshOption::None);
    }

    fn handle_backspace(&mut self) {
        let deleted = self.current_line().remove();
        self.cursor.x -= deleted.width_cjk().unwrap() as u16;
        self.refresh(RefreshOption::Line);
    }

    fn handle_upkey(&mut self) {
        self.move_up();
        self.refresh(RefreshOption::None);
    }

    fn handle_downkey(&mut self) {
        self.move_down();
        self.refresh(RefreshOption::None);
    }

    fn handle_leftkey(&mut self, refresh: bool) {
        self.current_line().prev();
        let char_width = self.current_line().current_char_width() as u16;
        self.cursor.move_left(char_width);

        if refresh {
            self.refresh(RefreshOption::None)
        }
    }

    fn handle_rightkey(&mut self, refresh: bool) {
        let char_width = self.current_line().current_char_width() as u16;
        self.cursor.move_right(char_width);
        let _no_use = self.current_line().next();

        if refresh {
            self.refresh(RefreshOption::None)
        }
    }

    fn handle_save(&self) {
        let mut file = File::create("./test.txt").unwrap();

        for s in &self.contents {
            writeln!(&mut file, "{}", s.get_buffer()).unwrap();
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_updown() {
        let mut ed = Editor::new();
        ed.current_line().push_str("가나다");
        ed.add_new_line();
        ed.current_line().push_str("abc");
        ed.cursor.x = 3;

        assert_eq!(ed.current_line().get_byte_index(), 3);

        ed.move_up();
        assert_eq!(ed.current_line().get_byte_index(), 3);
        assert_eq!(ed.cursor.x, 2);

        ed.move_down();
        assert_eq!(ed.current_line().get_byte_index(), 2);
        assert_eq!(ed.cursor.x, 2);
    }
}
