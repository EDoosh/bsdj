use crate::states;
use crate::tilerender::*;
use bevy::prelude::*;

pub mod input;
pub mod song_cursor;
pub mod types;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(types::TypeResourcePlugin);
        app.add_plugin(input::InputPlugin);
        app.insert_resource(song_cursor::SongCursor::new());
    }
}
