pub struct LineBuffer {
    buffer: String,
}

impl LineBuffer {
    pub fn new() -> LineBuffer {
        LineBuffer {
            buffer: String::new(),
        }
    }

    pub fn borrow(&mut self) -> &String {
        &self.buffer
    }

    pub fn push(&mut self, ch: char) {
        self.buffer.push(ch);
    }

    pub fn pop(&mut self) {
        self.buffer.pop();
    }
}
