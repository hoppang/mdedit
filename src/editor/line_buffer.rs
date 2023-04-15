use std::fmt;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, PartialEq)]
enum LineErr {
    EndOfString,
}

pub struct LineBuffer {
    s: String,
    cursor: usize,
}

impl LineBuffer {
    /*
    Constructors
    */

    pub fn new() -> LineBuffer {
        LineBuffer {
            s: String::new(),
            cursor: 0,
        }
    }

    pub fn from(arg: &str) -> LineBuffer {
        LineBuffer {
            s: String::from(arg),
            cursor: 0,
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

    pub fn len(&self) -> usize {
        self.s.len()
    }

    pub fn width(&self) -> usize {
        self.s.width_cjk()
    }

    /*
    Mutable functions
    */

    pub fn push(&mut self, ch: char) {
        self.s.push(ch);
    }

    pub fn pop(&mut self) {
        self.s.pop();
    }

    fn next(&mut self) -> Result<usize, LineErr> {
        loop {
            self.cursor += 1;

            if self.cursor >= self.s.len() {
                break Err(LineErr::EndOfString);
            }

            if self.s.is_char_boundary(self.cursor) {
                break Ok(self.cursor);
            }
        }
    }

    fn prev(&mut self) -> usize {
        loop {
            if self.cursor == 0 {
                break;
            }

            self.cursor -= 1;
            if self.cursor >= self.s.len() {
                continue;
            }

            if self.s.is_char_boundary(self.cursor) {
                break;
            }
        }

        self.cursor
    }
}

impl fmt::Debug for LineBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(\"{}\", {})", self.s, self.cursor)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let s: LineBuffer = LineBuffer::new();
        assert_eq!(s.s, "");
        assert_eq!(s.cursor, 0);
    }

    #[test]
    fn test_from() {
        let s: LineBuffer = LineBuffer::from("안녕");
        assert_eq!(s.s, "안녕");
        assert_eq!(s.cursor, 0);
    }

    #[test]
    fn test_cursor() {
        let mut s: LineBuffer = LineBuffer::from("돼지c");
        assert_eq!(s.next().unwrap(), 3);
        assert_eq!(s.next().unwrap(), 6);

        let result: i32 = match s.next() {
            Err(LineErr::EndOfString) => 1,
            _ => 2,
        };

        assert_eq!(result, 1);
        assert_eq!(s.cursor, s.len());

        assert_eq!(s.prev(), 6);
        assert_eq!(s.prev(), 3);
        assert_eq!(s.prev(), 0);
        assert_eq!(s.prev(), 0);
    }

    #[test]
    fn test_width() {
        let s1: LineBuffer = LineBuffer::from("Ｈｅｌｌｏ");
        assert_eq!(s1.width(), 10);

        let s2: LineBuffer = LineBuffer::from("김치stew");
        assert_eq!(s2.width(), 8);

        let s3: LineBuffer = LineBuffer::from("Hello");
        assert_eq!(s3.width(), 5);
    }
}
