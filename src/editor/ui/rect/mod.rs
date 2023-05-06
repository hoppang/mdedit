use crossterm::{cursor, queue};
use std::io::Stdout;

pub struct Rect {}

impl Rect {
    pub fn draw(screen: &Stdout, x: u16, y: u16, w: u16, h: u16) {
        Rect::draw_top_line(screen, x, y, w);
        Rect::draw_mid_lines(screen, x, y, w, h);
        Rect::draw_bottom_line(screen, x, y, w, h);
    }

    fn draw_top_line(mut screen: &Stdout, x: u16, y: u16, w: u16) {
        queue!(screen, cursor::MoveTo(x, y)).expect("draw_top_line failed");

        print!("╔");
        for _ in 2..(w) {
            print!("═");
        }
        print!("╗");
    }

    fn draw_mid_lines(mut screen: &Stdout, x: u16, y: u16, w: u16, h: u16) {
        for i in (y + 1)..(y + h - 1) {
            queue!(screen, cursor::MoveTo(x, i)).expect("Failed to move cursor (simple_dialog)");
            print!("║");
            for _ in (x + 1)..(x + w - 1) {
                print!(" ");
            }
            print!("║");
        }
    }

    fn draw_bottom_line(mut screen: &Stdout, x: u16, y: u16, w: u16, h: u16) {
        queue!(screen, cursor::MoveTo(x, y + h - 1))
            .expect("Failed to move cursor (simple_dialog)");
        print!("╚");
        for _ in 2..(w as i32) {
            print!("═");
        }
        print!("╝");
    }
}
