use bevy::prelude::*;

pub mod resize;
pub mod switch_appearance;

pub struct MetaActionsPlugin;

impl Plugin for MetaActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(resize::ResizePlugin);
        app.add_plugin(switch_appearance::SwitchAppearancePlugin);
    }
}
