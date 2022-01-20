use bevy::prelude::*;

pub mod chain;
pub mod channel;
pub mod command;
pub mod instrument;
pub mod note;
pub mod phrase;

pub use chain::Chains;
pub use channel::Channels;
pub use command::Command;
pub use instrument::Instruments;
pub use note::Note;
pub use phrase::Phrases;

pub struct TypeResourcePlugin;

impl Plugin for TypeResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Channels::default());
        app.insert_resource(Chains::default());
        app.insert_resource(Phrases::default());
        app.insert_resource(Instruments::default());
    }
}
