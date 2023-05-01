use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::{cursor, queue};
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
        let (width, height) = crossterm::terminal::size().unwrap();

        SimpleDialog {
            screen: scr,
            x: width / 4,
            y: height / 2 - 2,
            w: width / 2,
            h: 4,
        }
    }

    pub fn draw(&self, msg: String) {
        queue!(
            &self.screen,
            SetBackgroundColor(Color::White),
            SetForegroundColor(Color::Black)
        )
        .expect("Failed to move cursor (simple_dialog)");

        self.draw_top_line();
        self.draw_mid_lines();
        self.draw_bottom_line();
        self.draw_message(&msg);

        queue!(&self.screen, ResetColor).unwrap();
    }

    /**
     *  return: 팝업 종료 여부
     */
    pub fn handle_keyinput(&self, modifier: KeyModifiers, code: KeyCode) -> bool {
        matches!((modifier, code), (KeyModifiers::NONE, KeyCode::Esc))
    }

    fn draw_top_line(&self) {
        queue!(&self.screen, cursor::MoveTo(self.x, self.y)).expect("draw_top_line failed");

        print!("╔");
        for _ in 2..(self.w as i32) {
            print!("═");
        }
        print!("╗");
    }

    fn draw_mid_lines(&self) {
        for y in (self.y + 1)..(self.y + self.h - 1) {
            queue!(&self.screen, cursor::MoveTo(self.x, y))
                .expect("Failed to move cursor (simple_dialog)");
            print!("║");
            for _ in (self.x + 1)..(self.x + self.w - 1) {
                print!(" ");
            }
            print!("║");
        }
    }

    fn draw_bottom_line(&self) {
        queue!(&self.screen, cursor::MoveTo(self.x, self.y + self.h - 1))
            .expect("Failed to move cursor (simple_dialog)");
        print!("╚");
        for _ in 2..(self.w as i32) {
            print!("═");
        }
        print!("╝");
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
