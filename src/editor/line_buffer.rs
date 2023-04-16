use std::{cmp, fmt};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(Debug)]
pub enum LineErr {
    EndOfString,
}

impl std::cmp::PartialEq for LineErr {
    fn eq(&self, other: &LineErr) -> bool {
        self == other
    }
}

pub struct LineBuffer {
    s: String,
    byte_index: usize,
}

impl LineBuffer {
    /*
    Constructors
    */

    pub fn new() -> LineBuffer {
        LineBuffer {
            s: String::new(),
            byte_index: 0,
        }
    }

    #[cfg(test)]
    pub fn from(arg: &str) -> LineBuffer {
        LineBuffer {
            s: String::from(arg),
            byte_index: 0,
        }
    }

    /*
    Immutable functions
    */

    pub fn draw(&self, screen_width: usize) {
        print!("{}", self.s);

        for _ in self.s.len()..screen_width - 1 {
            print!(" ");
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.s.len()
    }

    pub fn width(&self) -> usize {
        self.s.width_cjk()
    }

    pub fn current_char(&self) -> char {
        self.get_char(self.byte_index)
    }

    fn get_char(&self, index: usize) -> char {
        let mut internal_cursor = index;
        loop {
            if self.s.is_char_boundary(internal_cursor) {
                break;
            }

            internal_cursor -= 1;
        }

        if self.s.is_empty() || internal_cursor >= self.s.len() {
            '\0'
        } else {
            let c = self.s[internal_cursor..].chars().next().unwrap();
            c
        }
    }

    pub fn current_char_width(&self) -> usize {
        if self.current_char() == '\0' {
            0
        } else {
            self.current_char().width_cjk().unwrap()
        }
    }

    /*
    Mutable functions
    */

    pub fn push(&mut self, ch: char) {
        self.s.push(ch);
        self.byte_index = self.s.len();
    }

    #[cfg(test)]
    pub fn push_str(&mut self, s: &str) {
        self.s.push_str(s);
        self.byte_index = self.s.len();
    }

    pub fn pop(&mut self) {
        self.s.pop();
    }

    pub fn next(&mut self) -> Result<usize, LineErr> {
        loop {
            self.byte_index += 1;
            self.byte_index = cmp::min(self.byte_index, self.s.len());

            if self.byte_index >= self.s.len() {
                break Err(LineErr::EndOfString);
            }

            if self.s.is_char_boundary(self.byte_index) {
                break Ok(self.byte_index);
            }
        }
    }

    pub fn prev(&mut self) -> usize {
        loop {
            if self.byte_index == 0 {
                break;
            }

            self.byte_index -= 1;
            if self.byte_index >= self.s.len() {
                continue;
            }

            if self.s.is_char_boundary(self.byte_index) {
                break;
            }
        }

        self.byte_index
    }

    /**
        이동키로 커서를 움직였을 때 새로운 byte index 설정이 필요하다.
        Return: (new_byte_index, new_screen_x_pos)
    */
    pub fn set_byte_index(&mut self, new_byte_index: u16) -> (u16, u16) {
        self.byte_index = new_byte_index as usize;

        loop {
            if self.s.is_char_boundary(self.byte_index) {
                break;
            }

            if self.byte_index > 0 {
                self.byte_index -= 1;
            }
        }

        let head_width = self.s[..self.byte_index].width_cjk();
        (self.byte_index as u16, head_width as u16)
    }

    // width 는 화면상의 길이, length 는 바이트 단위 길이
    pub fn index_from_width(&self, width: u16) -> u16 {
        let mut byte_index: u16 = 0;

        for c in self.s.chars() {
            if byte_index + 1 >= width {
                break;
            }

            byte_index += c.width_cjk().unwrap() as u16;
        }

        byte_index
    }

    pub fn get_byte_index(&self) -> usize {
        self.byte_index
    }
}

impl fmt::Debug for LineBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(\"{}\", {})", self.s, self.byte_index)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let s: LineBuffer = LineBuffer::new();
        assert_eq!(s.s, "");
        assert_eq!(s.byte_index, 0);
    }

    #[test]
    fn test_from() {
        let s: LineBuffer = LineBuffer::from("안녕");
        assert_eq!(s.s, "안녕");
        assert_eq!(s.byte_index, 0);
    }

    #[test]
    fn test_prev_next() {
        let mut s: LineBuffer = LineBuffer::from("돼지c");
        assert_eq!(s.next().unwrap(), 3);
        assert_eq!(s.next().unwrap(), 6);

        let result: i32 = match s.next() {
            Err(LineErr::EndOfString) => 1,
            _ => 2,
        };

        assert_eq!(result, 1);
        assert_eq!(s.byte_index, s.len());

        assert_eq!(s.prev(), 6);
        assert_eq!(s.prev(), 3);
        assert_eq!(s.prev(), 0);
        assert_eq!(s.prev(), 0);
    }

    #[test]
    fn test_str_width() {
        let s1: LineBuffer = LineBuffer::from("Ｈｅｌｌｏ");
        assert_eq!(s1.width(), 10);

        let s2: LineBuffer = LineBuffer::from("김치stew");
        assert_eq!(s2.width(), 8);

        let s3: LineBuffer = LineBuffer::from("Hello");
        assert_eq!(s3.width(), 5);
    }

    #[test]
    fn test_get() {
        let mut s1: LineBuffer = LineBuffer::from("Ｈｅｌｌｏ");
        s1.next().unwrap();
        assert_eq!(s1.current_char(), 'ｅ');

        let s2: LineBuffer = LineBuffer::from("안녕");
        assert_eq!(s2.current_char_width(), 2);
    }

    #[test]
    fn test_byte_index() {
        let mut s1: LineBuffer = LineBuffer::from("안녕하세요");
        s1.prev();
        assert_eq!(s1.current_char(), '안');

        // 커서가 char_boundary 위치가 아니더라도(한글 입력 중에 그렇게 될 수 있음)
        // 현재 캐릭터 정보를 가져올 수 있어야 한다.
        assert_eq!(s1.set_byte_index(5), (3, 2));
        assert_eq!(s1.set_byte_index(10), (9, 6));
        assert_eq!(s1.current_char(), '세');

        let mut s2: LineBuffer = LineBuffer::from("Hello");
        assert_eq!(s2.set_byte_index(3), (3, 3));
    }

    #[test]
    fn test_width_conv() {
        // unicode-width 를 bytes-length 로 변환하는 코드 테스트.
        let s: LineBuffer = LineBuffer::from("안녕하세요");

        assert_eq!(s.index_from_width(6), 6);
        assert_eq!(s.index_from_width(7), 6);
        assert_eq!(s.index_from_width(0), 0);
        assert_eq!(s.index_from_width(9), 8);
    }
}
