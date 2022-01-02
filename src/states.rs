#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum States {
    Project,
    Live,
    Song,
    Chain,
    Phrase,
    Instrument,
    Synth,
    Wave,
    Speech,
    Word,
    Table,
    Groove,
    File,
    Help(HelpScreen),
}

impl States {
    /// A 1-letter abbreviation of the state name.
    pub fn abbr(&self) -> &str {
        match self {
            States::Project => "p",
            States::Live => "l",
            States::Song => "s",
            States::Chain => "c",
            States::Phrase => "p",
            States::Instrument => "i",
            States::Synth => "s",
            States::Wave => "w",
            States::Speech => "i",
            States::Word => "w",
            States::Table => "t",
            States::Groove => "g",
            States::File => "f",
            States::Help(_) => "h",
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum HelpScreen {
    Main,
    Song,
    Live,
    Chain,
    Phrase,
    Instr,
    Groove,
    Wave,
    CommandList,
    MarkCopyPaste,
    MuteSoloPan,
}
