use super::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhraseCursorColumn {
    Note,
    Instrument,
    Command,
    CommandValue,
}

impl PhraseCursorColumn {
    pub fn to_num(self) -> u8 {
        match self {
            PhraseCursorColumn::Note => 0,
            PhraseCursorColumn::Instrument => 1,
            PhraseCursorColumn::Command => 2,
            PhraseCursorColumn::CommandValue => 3,
        }
    }

    pub fn from_num(num: u8) -> PhraseCursorColumn {
        match num {
            0 => PhraseCursorColumn::Note,
            1 => PhraseCursorColumn::Instrument,
            2 => PhraseCursorColumn::Command,
            3 => PhraseCursorColumn::CommandValue,
            _ => panic!(
                "Invalid number passed to PhraseCursorColumn::from_num: Expected 0-3, got {}",
                num
            ),
        }
    }
}

/// Indicates the position of the cursor on the phrase screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhraseCursor {
    /// The X position of the cursor
    x: u8,
    /// The Y position of the cursor
    y: u8,
}

impl PhraseCursor {
    /// Returns the column type.
    pub fn get_column(&self) -> PhraseCursorColumn {
        PhraseCursorColumn::from_num(self.get_x())
    }
}

impl Cursor for PhraseCursor {
    const MIN_X: isize = 0;
    const MAX_X: isize = 3;
    const MIN_Y: isize = 0;
    const MAX_Y: isize = 15;

    fn new() -> PhraseCursor {
        PhraseCursor { x: 0, y: 0 }
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
