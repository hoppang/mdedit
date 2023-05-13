use crossterm::queue;
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use log::error;

/**
 * 색상 설정 함수가 짧지만 더 짧게 만들려고 만든 유틸리티 함수
 */
pub fn set_color(fg_color: Color, bg_color: Color) {
    match queue!(
        std::io::stdout(),
        SetForegroundColor(fg_color),
        SetBackgroundColor(bg_color)
    ) {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to set_color: {:?}", e);
        }
    }
}
