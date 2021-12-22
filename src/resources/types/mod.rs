use crate::states;
use crate::tilerender::*;
use bevy::prelude::*;

pub mod channel;

pub struct TypeResourcePlugin;

impl Plugin for TypeResourcePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(channel::Channels::default());
    }
}
