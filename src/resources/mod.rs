use bevy::prelude::*;

pub mod edited_instrument;
pub mod input;
pub mod nav_cursor;
pub mod song_cursor;
pub mod types;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(types::TypeResourcePlugin);
        app.add_plugin(input::InputPlugin);
        app.insert_resource(song_cursor::SongCursor::new());
        app.insert_resource(nav_cursor::NavCursor::new());
        app.insert_resource(edited_instrument::EditedInstrument(0));
    }
}
