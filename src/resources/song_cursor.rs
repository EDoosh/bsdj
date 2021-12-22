use crate::utils::position::Position;

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
    /// Construct a new SongCursor at position 0, 0
    pub fn new() -> SongCursor {
        SongCursor { x: 0, y: 0, cam: 0 }
    }

    /// Retrieve the song cursor's position as a tuple
    pub fn get_pos(&self) -> (u8, u8) {
        (self.x, self.y)
    }
    /// Retrieve the song cursor's x position
    pub fn get_x(&self) -> u8 {
        self.x
    }
    /// Retrieve the song cursor's y position
    pub fn get_y(&self) -> u8 {
        self.y
    }
    /// Retrieve the song camera's position
    pub fn get_cam(&self) -> u8 {
        self.cam
    }

    /// Set the song cursor's x position.
    pub fn set_x(&mut self, x: isize) {
        self.x = self.clamp_x(x) as u8
    }
    /// Add one to the song cursor's x position.
    pub fn add_x(&mut self) {
        self.set_x(self.x as isize + 1)
    }
    /// Sub one to the song cursor's x position.
    pub fn sub_x(&mut self) {
        self.set_x(self.x as isize - 1)
    }

    /// Set the song cursor's y position.
    pub fn set_y(&mut self, y: isize) {
        self.y = self.clamp_y(y) as u8;

        // Update the camera
        if self.y < self.cam {
            self.cam = self.y
        } else if self.y > self.cam + 16 {
            self.cam = self.y - 16
        }
    }
    /// Add one to the song cursor's y position.
    pub fn add_y(&mut self) {
        self.set_y(self.y as isize + 1)
    }
    /// Sub one to the song cursor's y position.
    pub fn sub_y(&mut self) {
        self.set_y(self.y as isize - 1)
    }

    /// Set the song camera's position.
    pub fn set_cam(&mut self, cam: isize) {
        self.cam = std::cmp::max(0, std::cmp::min(240, cam)) as u8
    }
}

impl Position for SongCursor {
    const MIN_X: isize = 0;
    const MAX_X: isize = 3;
    const MIN_Y: isize = 0;
    const MAX_Y: isize = 255;
}
