use bevy::prelude::*;

pub mod resize;

pub struct MetaActionsPlugin;

impl Plugin for MetaActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(resize::ResizePlugin);
    }
}
