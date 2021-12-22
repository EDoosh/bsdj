#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
    Table,
    Groove,
    File,
    Help(HelpScreen),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
