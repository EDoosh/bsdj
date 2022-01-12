use bevy::prelude::*;

pub mod chain;
pub mod file;
pub mod groove;
pub mod help;
pub mod instrument;
pub mod live;
pub mod navbar;
pub mod phrase;
pub mod project;
pub mod sidebar;
pub mod song;
pub mod speech;
pub mod synth;
pub mod table;
pub mod wave;
pub mod word;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(project::ProjectScene);
        app.add_plugin(live::LiveScene);
        app.add_plugin(song::SongScene);
        app.add_plugin(chain::ChainScene);
        app.add_plugin(phrase::PhraseScene);
        app.add_plugin(instrument::InstrumentScene);
        app.add_plugin(synth::SynthScene);
        app.add_plugin(wave::WaveScene);
        app.add_plugin(speech::SpeechScene);
        app.add_plugin(word::WordScene);
        app.add_plugin(table::TableScene);
        app.add_plugin(groove::GrooveScene);
        app.add_plugin(file::FileScene);
        app.add_plugin(help::HelpScene);

        app.add_plugin(sidebar::SideBarPlugin);
        app.add_plugin(navbar::NavBarPlugin);
    }
}
