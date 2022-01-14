use bevy::prelude::*;

pub mod chain;
pub mod nav;
pub mod song;

pub use chain::ChainCursor;
pub use nav::NavCursor;
pub use song::SongCursor;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NavCursor::new());
        app.insert_resource(SongCursor::new());
        app.insert_resource(ChainCursor::new());
    }
}
