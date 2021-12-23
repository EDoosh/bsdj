use crate::tilerender::*;
use bevy::prelude::*;

pub mod navbar;
pub mod sidebar;
pub mod song;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(song::SongScene);
        app.add_plugin(sidebar::SideBarPlugin);
        app.add_plugin(navbar::NavBarPlugin);
    }
}
