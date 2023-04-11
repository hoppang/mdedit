use crossterm::terminal::size;

pub struct LineBuffer {
    buffer: String,
}

impl LineBuffer {
    pub fn new() -> LineBuffer {
        LineBuffer {
            buffer: String::new(),
        }
    }

    pub fn push(&mut self, ch: char) {
        self.buffer.push(ch);
    }

    pub fn pop(&mut self) {
        self.buffer.pop();
    }

    pub fn draw(&mut self) {
        print!("{}", self.buffer);

        let width = screen_width();
        for _ in self.buffer.len()..width - 1 {
            print!(" ");
        }
    }
}

fn screen_width() -> usize {
    match size() {
        Ok((cols, _rows)) => cols as usize,
        Err(error) => panic!("screen_width: {:?}", error),
    }
}
