// 0x00 to 0x40
pub const INSTR_COUNT: usize = 0x41;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruments {
    instrs: [Instrument; INSTR_COUNT],
}

impl Instruments {
    /// Get an instrument by its id.
    ///
    /// Returns None if the instrument is out of bounds (not within 0x00-0x40)
    pub fn get(&self, id: usize) -> Option<&Instrument> {
        self.instrs.get(id)
    }

    /// Gets a mutable instrument by its id.
    ///
    /// Returns None if the instrument is out of bounds (not within 0x00-0x40)
    pub fn get_mut(&mut self, id: usize) -> Option<&mut Instrument> {
        self.instrs.get_mut(id)
    }
}

impl Default for Instruments {
    fn default() -> Self {
        Instruments {
            instrs: [Instrument::default(); INSTR_COUNT],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instrument {
    Pulse(),
    Wave(),
    Kit(),
    Noise(),
    Speech(),
}

impl Instrument {
    /// Returns true if the instrument is a kit.
    pub fn is_kit(&self) -> bool {
        matches!(self, Instrument::Kit())
    }

    /// Returns true if the instrument is the speech instrument.
    pub fn is_speech(&self) -> bool {
        matches!(self, Instrument::Speech())
    }
}

impl Default for Instrument {
    fn default() -> Self {
        Instrument::Pulse()
    }
}
