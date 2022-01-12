/// The navbar position to go to.
/// None if there is no new scene to transition to.
pub struct NextState(pub Option<(u8, u8)>);

/// Used to indicate whether the scene should be loaded (or reloaded)
/// Reloading an already-loaded state with `State.overwrite_replace(new_state).unwrap()`
/// will error with the reason that it's already loaded, so our
/// workaround is to check at the beginning of each frame whether this value
/// is set or not and re-init accordingly.
pub struct LoadState(pub bool);

/// The current HelpScreen the user is on.
pub struct CurrentHelpScreen(pub HelpScreen);

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
    Help,
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
            States::Help => "h",
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum HelpScreen {
    Home,
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
