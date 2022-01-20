use bevy::prelude::*;

pub mod chain;
pub mod nav;
pub mod phrase;
pub mod song;

pub use chain::ChainCursor;
pub use nav::NavCursor;
pub use phrase::PhraseCursor;
pub use song::SongCursor;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NavCursor::new());
        app.insert_resource(SongCursor::new());
        app.insert_resource(ChainCursor::new());
        app.insert_resource(PhraseCursor::new());
    }
}

pub trait Cursor: std::marker::Sized {
    const MIN_X: isize;
    const MAX_X: isize;
    const MIN_Y: isize;
    const MAX_Y: isize;

    /// Construct a new cursor.
    fn new() -> Self;

    /// Clamp a value between the minimum and maximum permitted X.
    fn clamp_x(&mut self, x: isize) -> isize {
        x.clamp(Self::MIN_X, Self::MAX_X)
    }

    /// Clamp a value between the minimum and maximum permitted Y.
    fn clamp_y(&mut self, y: isize) -> isize {
        y.clamp(Self::MIN_Y, Self::MAX_Y)
    }

    /// Retrieve the cursor's position as a tuple
    fn get_pos(&self) -> (u8, u8);
    /// Retrieve the cursor's x position
    fn get_x(&self) -> u8;
    /// Retrieve the cursor's y position
    fn get_y(&self) -> u8;

    /// Set the cursor's x position.
    fn set_x(&mut self, x: isize);
    /// Add one to the cursor's x position.
    fn add_x(&mut self);
    /// Sub one to the cursor's x position.
    fn sub_x(&mut self);

    /// Set the cursor's y position.
    fn set_y(&mut self, y: isize);
    /// Add one to the cursor's y position.
    fn add_y(&mut self);
    /// Sub one to the cursor's y position.
    fn sub_y(&mut self);
}
