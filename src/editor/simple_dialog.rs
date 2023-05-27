use crate::consts::ui;
use crate::editor::ui::rect::Rect;
use crate::editor::util::set_color;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::style::ResetColor;
use crossterm::{cursor, queue};
use log::error;
use std::io::Stdout;
use unicode_width::UnicodeWidthStr;

pub struct SimpleDialog {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
    screen: Stdout,
}

impl SimpleDialog {
    pub fn new() -> SimpleDialog {
        let scr = std::io::stdout();
        let (width, height) = match crossterm::terminal::size() {
            Ok((width, height)) => (width, height),
            Err(e) => {
                error!("Make SimpleDialog with default size: {}", e);
                (40, 30)
            }
        };

        SimpleDialog {
            screen: scr,
            x: width / 4,
            y: height / 2 - 2,
            w: width / 2,
            h: 4,
        }
    }

    pub fn draw(&self, msg: String) {
        set_color(ui::DLG_BGCOLOR, ui::DLG_COLOR);

        Rect::draw(&self.screen, self.x, self.y, self.w, self.h);
        self.draw_message(&msg);

        queue!(&self.screen, ResetColor).unwrap();
    }

    /**
     *  return: 팝업 종료 여부
     */
    pub fn handle_keyinput(&self, modifier: KeyModifiers, code: KeyCode) -> bool {
        matches!((modifier, code), (KeyModifiers::NONE, KeyCode::Esc))
    }

    pub fn draw_message(&self, msg: &String) {
        let x_center = self.x + (self.w / 2);
        let x = x_center - (msg.width_cjk() / 2) as u16;

        queue!(&self.screen, cursor::MoveTo(x, self.y + 1)).expect("Failed to draw message");
        print!("{}", msg);

        let close_msg = String::from("Press ESC to close 😊");
        let btn_x = x_center - (close_msg.width_cjk() / 2) as u16;

        queue!(&self.screen, cursor::MoveTo(btn_x, self.y + 2)).expect("Failed to draw message");
        print!("{}", close_msg);
    }
}
