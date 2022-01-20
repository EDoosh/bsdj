use super::Command;
use crate::resources::types::note;

const EMPTY_INSTR: u8 = 0xff;
// 0x00 to 0xfe
const PHRASE_COUNT: usize = 0xFF;
// 16 notes, instruments, and commands per chain.
const NOTES_PER_CHAIN: usize = 0x10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phrases {
    phrases: [Phrase; PHRASE_COUNT],
}

impl Phrases {
    /// Get a phrase by its index.
    pub fn get(&self, index: usize) -> Option<&Phrase> {
        self.phrases.get(index)
    }

    /// Gets a mutable phrase by its index.
    ///
    /// Returns None if the specified index does not exist.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Phrase> {
        self.phrases.get_mut(index)
    }
}

impl Default for Phrases {
    fn default() -> Self {
        Phrases {
            phrases: [Phrase::default(); PHRASE_COUNT],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phrase {
    notes: [note::Note; NOTES_PER_CHAIN],
    instrs: [u8; NOTES_PER_CHAIN],
    cmds: [Command; NOTES_PER_CHAIN],
    cmd_vals: [u8; NOTES_PER_CHAIN],
}

impl Phrase {
    /// Returns a note at a position in the chain.
    /// Returns None if the note at that index is empty or if
    /// the index was out of bounds.
    pub fn get_note(&self, index: usize) -> Option<&note::Note> {
        let note = self.notes.get(index)?;
        if note.get().is_none() {
            None
        } else {
            Some(note)
        }
    }

    /// Sets the note at a position in the chain.
    /// Returns a None if the index was out of bounds.
    pub fn set_note(&mut self, index: usize, value: u8) -> Option<()> {
        *self.notes.get_mut(index)? = note::Note(value);
        Some(())
    }

    /// Clears the note at a position in the chain.
    /// Returns a None if the index was out of bounds.
    pub fn clear_note(&mut self, index: usize) -> Option<()> {
        self.set_note(index, note::EMPTY_NOTE)
    }

    /// Returns an instrument at a position in the chain.
    /// Returns None if the instrument at that index is empty or if
    /// the index was out of bounds.
    pub fn get_instr(&self, index: usize) -> Option<u8> {
        let instr = self.instrs.get(index);
        instr.filter(|i| **i != EMPTY_INSTR).cloned()
    }

    /// Sets the instrument at a position in the chain.
    /// Returns a None if the index was out of bounds.
    pub fn set_instr(&mut self, index: usize, value: u8) -> Option<()> {
        *self.instrs.get_mut(index)? = value;
        Some(())
    }

    /// Clears the instrument at a position in the chain.
    /// Returns a None if the index was out of bounds.
    pub fn clear_instr(&mut self, index: usize) -> Option<()> {
        self.set_instr(index, EMPTY_INSTR)
    }

    /// Returns a command at a position in the chain.
    /// Returns None if the command at that index is empty or if
    /// the index was out of bounds.
    pub fn get_cmd(&self, index: usize) -> Option<Command> {
        let cmd = self.cmds.get(index);
        cmd.filter(|c| **c != Command::None).cloned()
    }

    /// Sets the command at a position in the chain.
    /// Returns a None if the index was out of bounds.
    pub fn set_cmd(&mut self, index: usize, value: Command) -> Option<()> {
        *self.cmds.get_mut(index)? = value;
        Some(())
    }

    /// Clears the command at a position in the chain.
    /// Returns a None if the index was out of bounds.
    pub fn clear_cmd(&mut self, index: usize) -> Option<()> {
        self.set_cmd(index, Command::default())
    }

    /// Returns a command value at a position in the chain.
    /// Returns a None if the index was out of bounds.
    pub fn get_cmd_val(&self, index: usize) -> Option<u8> {
        self.cmd_vals.get(index).cloned()
    }

    /// Sets the command value at a position in the chain.
    /// Returns a None if the index was out of bounds.
    pub fn set_cmd_val(&mut self, index: usize, value: u8) -> Option<()> {
        *self.cmd_vals.get_mut(index)? = value;
        Some(())
    }
}

impl Default for Phrase {
    fn default() -> Phrase {
        Phrase {
            notes: [note::Note::default(); NOTES_PER_CHAIN],
            instrs: [EMPTY_INSTR; NOTES_PER_CHAIN],
            cmds: [Command::default(); NOTES_PER_CHAIN],
            cmd_vals: [0x00; NOTES_PER_CHAIN],
        }
    }
}
