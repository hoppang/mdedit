mod cursor;
mod line_buffer;
mod simple_dialog;
mod ui {
    pub mod menu_bar;
    pub mod rect;
}

use cursor::Cursor;
use line_buffer::LineBuffer;
use log::{error, info};
use simple_dialog::SimpleDialog;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Stdout, Write};
use ui::menu_bar::MenuBar;
use unicode_width::UnicodeWidthChar;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    terminal::{self, size, Clear, ClearType},
    Result,
};

enum RefreshOption {
    None,
    Line,
    Screen,
}

pub struct Editor {
    screen: Stdout,
    contents: Vec<LineBuffer>,
    cursor: Cursor,
    popup: Option<SimpleDialog>,
    menu_bar: MenuBar,
}

impl Editor {
    // 기본값
    pub fn new() -> Editor {
        info!("Create new editor object");

        let mut ed = Editor {
            screen: std::io::stdout(),
            cursor: Cursor::new(),
            contents: Vec::from([LineBuffer::new()]),
            popup: None,
            menu_bar: MenuBar::new(),
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
            let (modifier, code) = read_char().unwrap();

            // 글로벌 키
            match (modifier, code) {
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => break,
                (_, KeyCode::F(12)) => break,
                _ => {}
            }

            match &self.popup {
                None => self.handle_keyinput(modifier, code),
                Some(p) => {
                    let closed = p.handle_keyinput(modifier, code);
                    if closed {
                        self.popup = None;
                        self.refresh(RefreshOption::Screen)
                    }
                }
            }
        }

        execute!(&self.screen, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()
    }

    fn handle_keyinput(&mut self, modifier: KeyModifiers, code: KeyCode) {
        match (modifier, code) {
            (KeyModifiers::CONTROL, KeyCode::Char('s')) => self.handle_save(),
            (KeyModifiers::NONE, KeyCode::F(1)) => self.handle_help(),
            (KeyModifiers::NONE, KeyCode::F(10)) => self.handle_menu(),
            (_, KeyCode::Backspace) => self.handle_backspace(),
            (_, KeyCode::Char(c)) => self.handle_input_char(c),
            (KeyModifiers::NONE, KeyCode::Enter) => self.handle_enterkey(),
            (KeyModifiers::NONE, KeyCode::Left) => self.handle_leftkey(),
            (KeyModifiers::NONE, KeyCode::Right) => self.handle_rightkey(),
            (KeyModifiers::NONE, KeyCode::Up) => self.handle_upkey(),
            (KeyModifiers::NONE, KeyCode::Down) => self.handle_downkey(),
            _ => {} // do nothing
        }
    }

    fn open_file(&mut self, filename: &String) {
        self.contents.clear();

        let file = File::open(filename).unwrap();
        for line in io::BufReader::new(file).lines() {
            info!("line = {:?}", line);
            self.contents.push(LineBuffer::from(&line.unwrap()));
        }
    }

    /**
     * 현재 커서가 있는 한 줄 갱신
     */
    fn refresh(&mut self, opt: RefreshOption) {
        queue!(
            &self.screen,
            crossterm::cursor::MoveTo(0, self.cursor.screen_y())
        )
        .expect("Failed to move cursor");

        let mut line_count = 0;
        let screen_width = screen_width();

        match opt {
            RefreshOption::Line => {
                if let Some(line) = self.current_line() {
                    line.draw(screen_width)
                }
            }
            RefreshOption::Screen => {
                queue!(&self.screen, Clear(ClearType::All)).unwrap();

                for line in &self.contents {
                    info!(
                        "화면에 그리기: x {} y {} line {:?}",
                        line_count,
                        self.cursor.screen_y(),
                        line
                    );
                    queue!(&self.screen, crossterm::cursor::MoveTo(0, line_count))
                        .expect("Failed to move cursor");
                    line.draw(screen_width);
                    line_count += 1;
                }

                self.menu_bar.draw(&self.screen, screen_width);
            }
            _ => {}
        }

        // 디버그 정보 출력
        self.print_dbgmsg();

        queue!(
            &self.screen,
            crossterm::cursor::MoveTo(self.cursor.x, self.cursor.screen_y())
        )
        .expect("Failed to move cursor");

        match Write::flush(&mut self.screen) {
            Ok(()) => (),
            Err(error) => {
                panic!("Failed to put char {:?}", error);
            }
        };
    }

    fn print_dbgmsg(&mut self) {
        queue!(
            &self.screen,
            crossterm::cursor::MoveTo(0, screen_height() - 1)
        )
        .expect("Failed to move cursor");

        let x = self.cursor.x;
        let y = self.cursor.screen_y();
        print!(
            "current_line: {:?} cx {:?} cy {:?}",
            self.current_line(),
            x,
            y
        );
    }

    fn current_line(&mut self) -> Option<&mut LineBuffer> {
        if self.cursor.get_y() < self.contents.len() as u16 {
            Some(&mut self.contents[self.cursor.get_y() as usize])
        } else {
            None
        }
    }

    fn add_new_line(&mut self) {
        if self.cursor.get_y() < self.edit_area_height() {
            self.contents.push(LineBuffer::new());
            self.cursor.x = 0;
            self.cursor.move_down(self.edit_area_height());
        }
    }

    fn move_up(&mut self) {
        self.cursor.move_up();
        self.update_cursor_x();
    }

    fn move_down(&mut self) {
        self.cursor.move_down(self.edit_area_height());
        self.update_cursor_x();
    }

    fn update_cursor_x(&mut self) {
        let x = self.cursor.x as i32;
        let new_x = match self.current_line() {
            Some(line) => {
                let (new_x, new_byte_index) = line.cursor_and_byteindex(x);
                line.set_byte_index(new_byte_index);
                new_x
            }
            None => {
                error!("current_line is None: y {:?}", self.cursor.get_y());
                0
            }
        };

        self.cursor.x = new_x;
    }

    fn edit_area_height(&self) -> u16 {
        std::cmp::min(self.contents.len() as u16, screen_height() - 3)
    }

    // ================================================================================
    // 키 입력 핸들러

    fn handle_help(&mut self) {
        let dialog = SimpleDialog::new();
        dialog.draw(String::from(
            "mdedit: simple text editor inspired by MS-DOS EDIT",
        ));
        self.popup = Some(dialog);
        self.refresh(RefreshOption::None);
    }

    fn handle_menu(&mut self) {
        self.menu_bar.selected = Some(0);
        self.menu_bar.draw(&self.screen, screen_width());
        self.refresh(RefreshOption::None);
    }

    fn handle_input_char(&mut self, ch: char) {
        if let Some(line) = self.current_line() {
            line.insert(ch);
            self.cursor
                .move_right(screen_width() as u16, ch.width_cjk().unwrap() as u16);
            self.refresh(RefreshOption::Line);
        }
    }

    fn handle_enterkey(&mut self) {
        self.add_new_line();
        self.refresh(RefreshOption::None);
    }

    fn handle_backspace(&mut self) {
        if let Some(line) = self.current_line() {
            let deleted = line.remove();
            self.cursor.x -= deleted.width_cjk().unwrap() as u16;
            self.refresh(RefreshOption::Line);
        }
    }

    fn handle_upkey(&mut self) {
        self.move_up();
        self.refresh(RefreshOption::None);
    }

    fn handle_downkey(&mut self) {
        self.move_down();
        self.refresh(RefreshOption::None);
    }

    fn handle_leftkey(&mut self) {
        if let Some(line) = self.current_line() {
            line.prev();
            let char_width = line.current_char_width() as u16;
            self.cursor.move_left(char_width);
            self.refresh(RefreshOption::None);
        }
    }

    fn handle_rightkey(&mut self) {
        let char_width = match self.current_line() {
            Some(line) => {
                let char_width = line.current_char_width() as u16;
                let _no_use = line.next();

                char_width
            }
            None => 0,
        };
        self.cursor.move_right(screen_width() as u16, char_width);
        self.refresh(RefreshOption::None);
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
        ed.current_line().unwrap().push_str("가나다");
        ed.add_new_line();
        ed.current_line().unwrap().push_str("abc");
        ed.cursor.x = 3;

        assert_eq!(ed.current_line().unwrap().get_byte_index(), 3);

        ed.move_up();
        assert_eq!(ed.current_line().unwrap().get_byte_index(), 3);
        assert_eq!(ed.cursor.x, 2);

        ed.move_down();
        assert_eq!(ed.current_line().unwrap().get_byte_index(), 2);
        assert_eq!(ed.cursor.x, 2);
    }
}
