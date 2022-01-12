use bevy::prelude::*;

pub mod chain;
pub mod channel;

pub use chain::Chains;
pub use channel::Channels;

pub struct TypeResourcePlugin;

impl Plugin for TypeResourcePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(channel::Channels::default());
        app.insert_resource(chain::Chains::default());
    }
}
