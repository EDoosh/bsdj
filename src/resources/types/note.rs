pub const EMPTY_NOTE: u8 = 0x00;
// 12 notes per octave over 9 octaves.
// Only applicable to standard notes.
pub const LARGEST_NOTE: u8 = 12 * 9;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Note(pub u8);

impl Default for Note {
    fn default() -> Note {
        Note(EMPTY_NOTE)
    }
}

impl Note {
    /// Returns the value inside of a note, or none if the note is empty.
    pub fn get(self) -> Option<u8> {
        if self.0 == EMPTY_NOTE {
            None
        } else {
            Some(self.0)
        }
    }

    /// Converts a note to a string. Returns none if the note is not valid.
    ///
    /// ```
    /// use bsdj::resources::types::note::Note;
    ///
    /// // Returns none if the note is empty or too high.
    /// assert_eq!(None, Note(0).to_string(false));
    /// assert_eq!(None, Note(0xff).to_string(false));
    ///
    /// // The lowest note is C3
    /// assert_eq!("C 3", Note(1).to_string(false));
    /// // All notes are one octave lower if in a wave.
    /// assert_eq!("C 2", Note(1).to_string(true));
    /// ```
    pub fn to_string(self, is_wave: bool) -> Option<String> {
        let note = self.get()?;

        // Subtract 1 as the first Note value will be 1,
        // however 0 should be C3
        let note_str = match (note - 1) % 12 {
            0 => "c",
            1 => "c#",
            2 => "d",
            3 => "d#",
            4 => "e",
            5 => "f",
            6 => "f#",
            7 => "g",
            8 => "g#",
            9 => "a",
            10 => "a#",
            11 => "b",
            _ => panic!("note % 12 greater than 11?"),
        };
        let mut octave = (note - 1) / 12;
        // In Pu1, Pu2, and Noi, octaves display 3-B, so add 3 to the octave.
        // But in a wave, they display 2-A, so add 2.
        octave += if is_wave { 2 } else { 3 };

        // Overwrite the previous default note text with the
        // note and octave.
        Some(format!("{:2}{:x}", note_str, octave))
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    /// Test basics of creating and stringifying a note are working.
    fn basic_to_string() {
        let note = Note(17);
        assert_eq!(Some("C 3"), note.to_string(false));
    }

    #[test]
    /// Create a note with no value and try convert it to a string.
    fn empty_to_string() {
        let note = Note(EMPTY_NOTE);
        assert_eq!(None, note.to_string(false));
    }

    #[test]
    /// Create a note with the lowest possible value and try convert it to a string.
    fn lowest_to_string() {
        let note = Note(1);
        assert_eq!(Some("C 3"), note.to_string(false));

        // As a wave
        let note = Note(1);
        assert_eq!(Some("C 2"), note.to_string(true));
    }

    #[test]
    /// Create a note with the highest valid value and try convert it to a string.
    fn highest_to_string() {
        let note = Note(108);
        assert_eq!(Some("B B"), note.to_string(false));

        // As a wave
        let note = Note(108);
        assert_eq!(Some("B A"), note.to_string(true));
    }

    #[test]
    /// Create a note with one more than the highest valid value and try convert it to a string.
    fn invalid_to_string() {
        let note = Note(109);
        assert_eq!(None, note.to_string(false));

        // As a wave
        let note = Note(109);
        assert_eq!(None, note.to_string(true));
    }

    #[test]
    /// Test getting a note's value.
    fn note_get() {
        let note = Note(15);
        assert_eq!(Some(15), note.get());

        // Test as max.
        let note = Note(0xff);
        assert_eq!(Some(0xff), note.get());
    }

    #[test]
    /// Test a note with no value.
    fn empty_get() {
        let note = Note(EMPTY_NOTE);
        assert_eq!(None, note.get())
    }
}
