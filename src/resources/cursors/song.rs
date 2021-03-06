use super::Cursor;

/// Indicates the position of the cursor on the main Song screen.
pub struct SongCursor {
    /// The X position of the cursor
    x: u8,
    /// The Y position of the cursor
    y: u8,
    /// The current X position of the camera (from the top)
    cam: u8,
}

impl SongCursor {
    /// Retrieve the song camera's position
    pub fn get_cam(&self) -> u8 {
        self.cam
    }

    /// Set the song camera's position.
    pub fn set_cam(&mut self, cam: isize) {
        self.cam = std::cmp::max(0, std::cmp::min(240, cam)) as u8
    }

    // CUSTOM FUNCTIONS
    /// Returns true if the song cursor is currently in the wave channel.
    pub fn is_wav(&self) -> bool {
        self.x == 2
    }
}

impl Cursor for SongCursor {
    const MIN_X: isize = 0;
    const MAX_X: isize = 3;
    const MIN_Y: isize = 0;
    const MAX_Y: isize = 255;

    fn new() -> SongCursor {
        SongCursor { x: 0, y: 0, cam: 0 }
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

        // Update the camera
        if self.y < self.cam {
            self.cam = self.y
        } else if self.y > self.cam + 15 {
            self.cam = self.y - 15
        }
    }
    fn add_y(&mut self) {
        self.set_y(self.y as isize + 1)
    }
    fn sub_y(&mut self) {
        self.set_y(self.y as isize - 1)
    }
}
