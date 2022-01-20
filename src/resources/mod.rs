use bevy::prelude::*;

pub mod cursors;
pub mod edited;
pub mod input;
pub mod types;

pub use cursors::Cursor;

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(types::TypeResourcePlugin);
        app.add_plugin(input::InputPlugin);
        app.add_plugin(edited::EditedPlugin);
        app.add_plugin(cursors::CursorPlugin);
    }
}
