use super::*;
use crate::resources::{input::*, *};
use crate::states;
use crate::tilerender::*;
use crate::utils;
use bevy::prelude::*;

pub struct ChainScene;

impl Plugin for ChainScene {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(states::States::Chain).with_system(enter_scene));
        app.add_system_set(
            SystemSet::on_update(states::States::Chain)
                // .with_system(handle_scroll)
                // .with_system(move_cursor)
                // .with_system(type_chain)
                // .with_system(delete_chain_system)
                // .with_system(bookmark_chain_system)
                // .with_system(open_chain_system)
                .with_system(draw_screen),
        );
        // app.add_system_set(
        //     SystemSet::on_exit(states::States::Chain).with_system(exit_game),
        // );
    }
}

fn enter_scene(mut lh: ResMut<LayerHandler>, edited_chain: Res<edited_chain::EditedChain>) {
    // Clear the map
    lh.clear_layer("map", "space", colors::Colors::Background)
        .unwrap();
    // Set the top of the screen to say "CHAIN XX"
    lh.set_tiles_string("map", 0, 0, "chain", colors::Colors::Background)
        .unwrap();
    lh.set_tiles_hex(
        "map",
        6,
        0,
        edited_chain.0 as usize,
        2,
        colors::Colors::Background,
    )
    .unwrap();
    // Set the tiles underneath to say "PHR" and "TSP"
    lh.set_tiles(
        "map",
        2,
        1,
        &["phr1", "phr2", "", "tsp1", "tsp2"],
        colors::Colors::Background,
    )
    .unwrap();

    // Write the phrase indexes on the side
    for y in 0..16 {
        lh.set_tiles_hex("map", 0, 2 + y, y, 1, colors::Colors::Details)
            .unwrap();
    }
}

fn draw_screen(mut lh: ResMut<LayerHandler>) {
    for y in 0..16 {
        // Draw the phrase
        // Default text and color
        let text = "--".to_string();
        let color = colors::Colors::Background;

        // let color = if chain_cursor.get_pos() == (1, y as u8) {
        //     colors::Colors::Cursor
        // } else {
        //      colors::Colors::Background
        // }

        // // Get the phrase and adjust its text content.
        // let text = if let Some(phrase) = chain.get_phrase(y) {
        //     format!("{:02x}", phrase)
        // } else {
        //      "--".to_string()
        // }

        // Set the tiles.
        lh.set_tiles_string("map", 2, y + 2, &text, color).unwrap();

        // Draw the transpose.
        lh.set_tiles_hex("map", 5, y + 2, 0, 2, colors::Colors::Highlight)
            .unwrap();
    }
}
