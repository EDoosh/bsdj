// use crate::resources::{input::*, *};
use crate::states;
use crate::tilerender::*;
use bevy::prelude::*;

pub struct SynthScene;

impl Plugin for SynthScene {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(states::States::Synth).with_system(enter_scene), // .with_system(draw_screen),
        );
    }
}

fn enter_scene(mut lh: ResMut<LayerHandler>, load_scene: ResMut<states::LoadState>) {
    // Dont try enter the scene if the scene should not be loaded.
    if !load_scene.0 {
        return;
    }

    // Clear the map
    lh.clear_layer("map", "space", Colors::Background).unwrap();

    // Show a 'TO IMPLEMENT' error.
    lh.set_tiles_string("map", 2, 4, "-------------", Colors::Highlight)
        .unwrap();
    lh.set_tiles_string("map", 2, 5, "    error:   ", Colors::Highlight)
        .unwrap();
    lh.set_tiles_string("map", 2, 6, "    synth    ", Colors::Highlight)
        .unwrap();
    lh.set_tiles_string("map", 2, 7, "   not yet   ", Colors::Highlight)
        .unwrap();
    lh.set_tiles_string("map", 2, 8, " implemented ", Colors::Highlight)
        .unwrap();
    lh.set_tiles_string("map", 2, 9, "-------------", Colors::Highlight)
        .unwrap();
}

// fn draw_screen(mut lh: ResMut<LayerHandler>) {}
