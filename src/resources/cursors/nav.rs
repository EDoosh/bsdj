use super::Cursor;

/// Indicates the position of the cursor on the navbar.
pub struct NavCursor {
    /// The X position of the cursor
    x: u8,
    /// The Y position of the cursor
    y: u8,
}

impl Cursor for NavCursor {
    const MIN_X: isize = 0;
    const MAX_X: isize = 4;
    const MIN_Y: isize = 0;
    const MAX_Y: isize = 2;

    fn new() -> NavCursor {
        NavCursor { x: 0, y: 1 }
    }

    fn get_pos(&self) -> (u8, u8) {
        (self.x, self.y)
    }
    fn get_x(&self) -> u8 {
        self.x
    }
    fn get_y(&self) -> u8 {
        self.y
    }

    fn set_x(&mut self, x: isize) {
        self.x = self.clamp_x(x) as u8
    }
    fn add_x(&mut self) {
        self.set_x(self.x as isize + 1)
    }
    fn sub_x(&mut self) {
        self.set_x(self.x as isize - 1)
    }

    fn set_y(&mut self, y: isize) {
        self.y = self.clamp_y(y) as u8;
    }
    fn add_y(&mut self) {
        self.set_y(self.y as isize + 1)
    }
    fn sub_y(&mut self) {
        self.set_y(self.y as isize - 1)
    }
}
