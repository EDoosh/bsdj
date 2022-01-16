// 0x00 to 0x40
const INSTR_COUNT: usize = 0x41;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruments {
    instrs: [Instrument; INSTR_COUNT],
}

impl Instruments {
    /// Get an instrument by its index.
    pub fn get(&self, index: usize) -> Option<&Instrument> {
        self.instrs.get(index)
    }

    /// Gets a mutable instrument by its index.
    ///
    /// Returns None if the specified index does not exist.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Instrument> {
        self.instrs.get_mut(index)
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
