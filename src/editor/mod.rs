mod cursor;
mod line_buffer;
mod simple_dialog;
mod util;
mod ui {
    pub mod menu_bar;
    pub mod rect;
}

use crate::check_result;
use crate::consts::ui::MenuCmd;
use cursor::Cursor;
use line_buffer::LineBuffer;
use log::{error, info};
use queues::*;
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
    cmd_queue: Queue<MenuCmd>,
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
            cmd_queue: Queue::new(),
        };

        let args: Vec<String> = env::args().collect();
        if args.len() == 2 {
            info!("Open file {:?}", args[1]);
            ed.open_file(&args[1])
        }

        ed
    }

    /**
        에디터의 메인 루프

        # Return
        * main 함수의 리턴값으로 Ok 를 리턴
    */
    pub fn run(&mut self) -> Result<()> {
        execute!(&self.screen, terminal::EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        self.refresh(RefreshOption::Screen);

        loop {
            // command queue 처리. 하나씩
            if self.cmd_queue.size() > 0 {
                if let Ok(cmd) = self.cmd_queue.peek() {
                    match self.cmd_queue.remove() {
                        Ok(_) => match cmd {
                            MenuCmd::Exit => self.goodbye(),
                            MenuCmd::About => self.handle_help(),
                            _ => {}
                        },
                        Err(e) => error!("Failed to remove cmd from queue: {}", e),
                    }
                }
            }

            let (modifier, code) = match read_char() {
                Ok((m, c)) => match (m, c) {
                    // 글로벌 키 처리
                    (KeyModifiers::CONTROL, KeyCode::Char('q')) => break,
                    (_, KeyCode::F(12)) => break,
                    _ => (m, c),
                },
                Err(_) => break,
            };

            if self.menu_bar.selected.is_some() {
                let cmd = self.menu_bar.handle_keyinput(modifier, code);

                match cmd {
                    MenuCmd::CloseMenu => {
                        self.menu_bar.selected = None;
                        self.refresh(RefreshOption::Screen);
                    }
                    MenuCmd::Refresh => {
                        self.refresh(RefreshOption::Screen);
                    }
                    _ => {
                        match self.cmd_queue.add(cmd) {
                            Ok(_) => {}
                            Err(e) => error!("cmd_queue add error: {}", e),
                        };
                        self.menu_bar.selected = None;
                        self.refresh(RefreshOption::Screen);
                    }
                }

                continue;
            }

            match &self.popup {
                None => self.handle_keyinput(modifier, code),
                Some(p) => {
                    if p.handle_keyinput(modifier, code) {
                        self.popup = None;
                        self.refresh(RefreshOption::Screen)
                    }
                }
            }
        }

        self.goodbye();
        Ok(())
    }

    pub fn goodbye(&self) {
        execute!(&self.screen, terminal::LeaveAlternateScreen).unwrap();
        check_result!(terminal::disable_raw_mode(), "Unable to disable raw mode");
        std::process::exit(0);
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

    /**
        파일을 열고 내용을 읽어들인다.

        # Arguments
        * `filename` - 파일 이름
    */
    fn open_file(&mut self, filename: &String) {
        self.contents.clear();

        match File::open(filename) {
            Ok(file) => {
                for line in io::BufReader::new(file).lines() {
                    info!("line = {:?}", line);
                    match line {
                        Ok(l) => self.contents.push(LineBuffer::from(&l)),
                        Err(e) => error!("Failed to read line: {}", e),
                    }
                }
            }
            Err(e) => error!("Failed to open file: {}", e),
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

        check_result!(Write::flush(&mut self.screen), "Failed to put char");
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

    /**
    입력된 키가 일반 문자일 경우 처리

    # Arguments
    * `ch` - 입력된 문자
    */
    fn handle_input_char(&mut self, ch: char) {
        if let Some(line) = self.current_line() {
            line.insert(ch);
            self.cursor
                .move_right(screen_width() as u16, ch.width_cjk().unwrap_or(0) as u16);
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
            let char_width = match deleted.width_cjk() {
                Some(width) => width as u16,
                None => {
                    error!("Unexpected operation on width_cjk");
                    0
                }
            };
            self.cursor.x -= char_width;
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
        match File::create("./test.txt") {
            Ok(mut file) => {
                for s in &self.contents {
                    match writeln!(&mut file, "{}", s.get_buffer()) {
                        Ok(_) => {}
                        Err(e) => error!("Failed to write line {}", e),
                    }
                }
            }
            Err(e) => {
                error!("Failed to open file to save: {:?}", e);
            }
        }
    }
}

fn read_char() -> Result<(KeyModifiers, KeyCode)> {
    loop {
        // rust 의 char 크기는 4바이트이므로 한글도 들어감.
        if event::poll(std::time::Duration::from_millis(25))? {
            if let Ok(Event::Key(KeyEvent {
                code: c,
                modifiers: m,
            })) = event::read()
            {
                return Ok((m, c));
            }
        }
    }
}

fn screen_width() -> usize {
    match size() {
        Ok((cols, _rows)) => cols as usize,
        Err(error) => {
            error!("screen_width: {:?}", error);
            0
        }
    }
}

fn screen_height() -> u16 {
    match size() {
        Ok((_cols, rows)) => rows,
        Err(error) => {
            error!("screen_height: {:?}", error);
            0
        }
    }
}

/*
    todo: github actions 에서 사용할 수 있는 테스트로 변경
    에러메시지:
        thread 'editor::test::test_move_updown' panicked at 'screen_width: Os { code: 11, kind: WouldBlock, message: "Resource temporarily unavailable" }', src/editor/mod.rs:390:23
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
*/
