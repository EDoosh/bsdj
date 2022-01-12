use crate::utils::position::Position;

/// Indicates the position of the cursor on the chain screen.
pub struct ChainCursor {
    /// The X position of the cursor
    x: u8,
    /// The Y position of the cursor
    y: u8,
}

impl ChainCursor {
    /// Construct a new ChainCursor at position 0, 0 (The first phrase)
    pub fn new() -> ChainCursor {
        ChainCursor { x: 0, y: 0 }
    }

    /// Retrieve the chain cursor's position as a tuple
    pub fn get_pos(&self) -> (u8, u8) {
        (self.x, self.y)
    }
    /// Retrieve the chain cursor's x position
    pub fn get_x(&self) -> u8 {
        self.x
    }
    /// Retrieve the chain cursor's y position
    pub fn get_y(&self) -> u8 {
        self.y
    }

    /// Set the chain cursor's x position.
    pub fn set_x(&mut self, x: isize) {
        self.x = self.clamp_x(x) as u8
    }
    /// Add one to the chain cursor's x position.
    pub fn add_x(&mut self) {
        self.set_x(self.x as isize + 1)
    }
    /// Sub one to the chain cursor's x position.
    pub fn sub_x(&mut self) {
        self.set_x(self.x as isize - 1)
    }

    /// Set the chain cursor's y position.
    pub fn set_y(&mut self, y: isize) {
        self.y = self.clamp_y(y) as u8;
    }
    /// Add one to the chain cursor's y position.
    pub fn add_y(&mut self) {
        self.set_y(self.y as isize + 1)
    }
    /// Sub one to the chain cursor's y position.
    pub fn sub_y(&mut self) {
        self.set_y(self.y as isize - 1)
    }
}

impl Position for ChainCursor {
    const MIN_X: isize = 0;
    const MAX_X: isize = 1;
    const MIN_Y: isize = 0;
    const MAX_Y: isize = 15;
}
