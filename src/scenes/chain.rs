use crate::resources::{input::*, *};
use crate::states;
use crate::tilerender::*;
use bevy::prelude::*;

pub struct ChainScene;

impl Plugin for ChainScene {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(states::States::Chain)
                .with_system(enter_scene)
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

fn enter_scene(
    mut lh: ResMut<LayerHandler>,
    edited_chain: Res<edited::EditedChain>,
    mut load_scene: ResMut<states::LoadState>,
) {
    // Dont try enter the scene if the scene should not be loaded.
    if !load_scene.0 {
        return;
    }
    // Make sure not to reload next scene.
    load_scene.0 = false;

    // Clear the map
    lh.clear_layer("map", "space", Colors::Background).unwrap();

    // Set the top of the screen to say "CHAIN XX"
    lh.set_tiles_string("map", 0, 0, "chain", Colors::Background)
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

fn handle_scroll(
    input: Res<InputRes>,
    edited_chain: Res<edited::EditedChain>,
    mut chains: ResMut<types::chain::Chains>,
) {
    let scroll_delta = input.get_scroll_delta();

    // Scroll wheel
    if scroll_delta == 0 {
        return;
    }

    // Require control key to be pressed to change value of phrase or transpose.
    if !input.exclusively_pressed(&[InputType::Key(KeyCode::LControl)]) {
        return;
    }

    // if let Some(cursor_pos) = input.get_cursor_tile_position() {
    //     if let Some((is_chain, index, left)) = hover(cursor_pos) {
    //         // Get the chain info
    //         let chain = chains.get_mut(edited_chain.0 as usize);
    //         let current = chain.get_chain(chain_y);

    //         let change = if left { 0x10 } else { 0x01 };

    //         // If there is no chain there already, set to 0.
    //         // Else add the change, but clamp between 0 and 0x7f
    //         let mut new = 0;
    //         if let Some(chain) = current {
    //             new = chain as i32 + change * scroll_delta;
    //             new = new.clamp(0, 0x7f)
    //         }

    //         channel.set_chain(chain_y, new as u8);
    //     }
    // }
}

/// Determines where the user cursor is on an inputtable value.
/// Returns an Option. None means no inputtable value is hovered.
///
/// The first parameter of the tuple will be whether a chain or transpose is hovered.
/// The second parameter will be the index hovered.
/// The third paramter will indicate whether the left value was hovered.
fn hover(cursor_pos: (i32, i32)) -> Option<(bool, u8, bool)> {
    let (x, y) = cursor_pos;
    if y < 2 {
        return None;
    }

    let left;
    if [2, 5].contains(&x) {
        // Left column
        left = true;
    } else if [3, 6].contains(&x) {
        // Right column
        left = false;
    } else {
        // No chain column
        return None;
    }

    let is_chain = x < 4;
    let index = y as u8 - 2;
    Some((is_chain, index, left))
}

fn draw_screen(
    mut lh: ResMut<LayerHandler>,
    chain_cursor: Res<cursors::ChainCursor>,
    chains: Res<types::Chains>,
    edited_chain: Res<edited::EditedChain>,
) {
    let chain = chains.get(edited_chain.0 as usize);

    // Show an error if the chain doesn't exist.
    if chain.is_none() {
        lh.set_tiles_string("map", 4, 4, "----------", Colors::Highlight)
            .unwrap();
        lh.set_tiles_string("map", 4, 5, "  error:  ", Colors::Highlight)
            .unwrap();
        lh.set_tiles_string("map", 4, 6, " invalid  ", Colors::Highlight)
            .unwrap();
        lh.set_tiles_string("map", 4, 7, " chain id ", Colors::Highlight)
            .unwrap();
        lh.set_tiles_string("map", 4, 8, "----------", Colors::Highlight)
            .unwrap();
        return;
    }

    let chain = chain.unwrap();
    let chain_cursor_pos = chain_cursor.get_pos();

    for y in 0..16 {
        // Draw the phrase
        // Set color if cursor is currently there.
        let color = if chain_cursor_pos == (0, y as u8) {
            colors::Colors::Cursor
        } else {
            colors::Colors::Background
        };

        // Get the phrase and adjust its text content.
        let text = if let Some(phrase) = chain.get_phrase(y) {
            format!("{:02x}", phrase)
        } else {
            "--".to_string()
        };

        // Set the tiles.
        lh.set_tiles_string("map", 2, y + 2, &text, color).unwrap();

        // Draw the transpose.
        lh.set_tiles_hex("map", 5, y + 2, 0, 2, colors::Colors::Highlight)
            .unwrap();
    }
}
