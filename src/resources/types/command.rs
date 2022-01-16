use std::fmt;

/// An enum of all valid Command, each also holding
/// the value of the command.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Command {
    None,
    Table,
    Chord,
    Delay,
    Envelope,
    Frame,
    Groove,
    Hop,
    Kill,
    Legato,
    Master,
    Pan,
    Pitch,
    Retrigger,
    Sweep,
    Tempo,
    Vibrato,
    Wave,
    Randomize,
}

impl Command {
    /// Returns a command from a given hex number.
    pub fn from_num(num: u8) -> Result<Command, String> {
        match num {
            0x00 => Ok(Command::None),
            0x01..=0x12 => Ok(Command::iter()[num as usize]),
            _ => Err(format!(
                "Command num invalid: Expected 0x00-0x12, got 0x{:02X}",
                num
            )),
        }
    }

    pub fn array_from_num(cmds: &[u8]) -> Result<Vec<Command>, String> {
        let mut new_cmds = vec![];
        for cmd in cmds {
            new_cmds.push(Command::from_num(*cmd)?)
        }
        Ok(new_cmds)
    }

    /// Returns a number given a command. Used for storing back into the SAV file.
    pub fn to_num(self) -> u8 {
        if self == Command::None {
            0x00
        } else {
            Command::iter().iter().position(|&cmd| cmd == self).unwrap() as u8
        }
    }

    /// Get the single char abbreviation of the command
    pub fn get_abbr(&self) -> Option<char> {
        match self {
            Command::None => None,
            Command::Table => Some('a'),
            Command::Chord => Some('c'),
            Command::Delay => Some('d'),
            Command::Envelope => Some('e'),
            Command::Frame => Some('f'),
            Command::Groove => Some('g'),
            Command::Hop => Some('h'),
            Command::Kill => Some('k'),
            Command::Legato => Some('l'),
            Command::Master => Some('m'),
            Command::Pan => Some('o'),
            Command::Pitch => Some('p'),
            Command::Retrigger => Some('r'),
            Command::Sweep => Some('s'),
            Command::Tempo => Some('t'),
            Command::Vibrato => Some('v'),
            Command::Wave => Some('w'),
            Command::Randomize => Some('z'),
        }
    }

    /// Returns an array of all Command (excluding None)
    /// Returned in abbreviation alphabetical order.
    pub fn iter() -> [Command; 18] {
        [
            Command::Table,
            Command::Chord,
            Command::Delay,
            Command::Envelope,
            Command::Frame,
            Command::Groove,
            Command::Hop,
            Command::Kill,
            Command::Legato,
            Command::Master,
            Command::Pan,
            Command::Pitch,
            Command::Retrigger,
            Command::Sweep,
            Command::Tempo,
            Command::Vibrato,
            Command::Wave,
            Command::Randomize,
        ]
    }

    /// Returns a string of the command's value.
    /// If the command is Command::Pan, returns an LR string.
    /// If the command is Command::Wave, returns a string representation of the wave.
    /// In all other cases, returns a 2-digit hex string of the value of the command.
    pub fn get_val_str(&self, val: u8) -> String {
        match self {
            Command::Pan => match val % 4 {
                0 => "  ",
                1 => "L ",
                2 => " R",
                3 => "LR",
                _ => panic!("val % 4 not in range 0-3????"),
            }
            .to_string(),
            Command::Wave => match val % 4 {
                0 => "L_",
                1 => "𝕃_",
                2 => "П_",
                3 => "Гl",
                _ => panic!("val % 4 not in range 0-3????"),
            }
            .to_string(),
            _ => format!("{:02X}", val),
        }
    }
}

impl Default for Command {
    fn default() -> Self {
        Command::None
    }
}

impl fmt::Display for Command {
    /// Returns a single-letter abbreviation next to the value the command has.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_abbr().unwrap_or('-'))
    }
}
