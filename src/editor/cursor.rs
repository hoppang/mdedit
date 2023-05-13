pub struct Cursor {
    pub x: u16,
    y: u16,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor { x: 0, y: 0 }
    }

    pub fn move_left(&mut self, x: u16) {
        if x <= self.x {
            self.x -= x;
        }
    }

    pub fn move_right(&mut self, width: u16, x: u16) {
        if self.x + x <= width {
            self.x += x;
        }
    }

    pub fn move_up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn move_down(&mut self, height: u16) {
        if self.y < height {
            self.y += 1;
        }
    }

    pub fn get_y(&self) -> u16 {
        self.y
    }

    pub fn screen_y(&self) -> u16 {
        self.y + 1
    }
}
