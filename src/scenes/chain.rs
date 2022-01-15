use crate::resources::{input::*, *};
use crate::states;
use crate::tilerender::*;
use crate::utils::u8_utils::WrappingAdd;
use bevy::prelude::*;

pub struct ChainScene;

impl Plugin for ChainScene {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(states::States::Chain)
                .with_system(enter_scene)
                .with_system(handle_scroll)
                .with_system(move_cursor)
                .with_system(type_value)
                .with_system(delete_value_system)
                .with_system(open_phrase_system)
                .with_system(draw_screen),
        );
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

    if let Some(cursor_pos) = input.get_cursor_tile_position() {
        if let Some((is_chain, index, is_left)) = hover(cursor_pos) {
            // Get the chain info
            if let Some(chain) = chains.get_mut(edited_chain.0 as usize) {
                let change = if is_left { 0x10 } else { 0x01 };

                if is_chain {
                    // If there is no chain there already, set to 0.
                    // Else add the change, but clamp between 0 and 0xfe
                    let mut new = 0;
                    if let Some(phrase) = chain.get_phrase(index as usize) {
                        new = phrase as i32 + change * scroll_delta;
                        new = new.clamp(0, 0xfe)
                    }
                    chain.set_phrase(index as usize, new as u8);
                } else {
                    // Editing the transpose.
                    // Add to the transpose, overflowing/underflowing
                    // if necessary.
                    if let Some(transpose) = chain.get_transpose(index as usize) {
                        let new = transpose.w_add((change * scroll_delta) as isize);
                        chain.set_transpose(index as usize, new);
                    }
                }
            }
        }
    }
}

fn move_cursor(
    input: Res<InputRes>,
    mut chain_cursor: ResMut<cursors::ChainCursor>,
    chains: Res<types::chain::Chains>,
    edited_chain: ResMut<edited::EditedChain>,
    mut edited_phrase: ResMut<edited::EditedPhrase>,
) {
    // Move the cursor to the point the user clicked.
    if input.just_pressed(&InputType::Mouse(MouseButton::Left)) {
        if let Some(cursor_pos) = input.get_cursor_tile_position() {
            if let Some((is_chain, index, _)) = hover(cursor_pos) {
                chain_cursor.set_x(if is_chain { 0 } else { 1 });
                chain_cursor.set_y(index as isize);
            }
        }
    }

    // Move the cursor based on directional inputs (Up, Left, Down, and Right)
    for key in InputType::directional_keycodes() {
        if input.dr_pressed(&key) && input.exclusively_pressed(&[key]) {
            match key {
                InputType::Key(KeyCode::Up) => chain_cursor.sub_y(),
                InputType::Key(KeyCode::Down) => chain_cursor.add_y(),
                InputType::Key(KeyCode::Left) => chain_cursor.sub_x(),
                InputType::Key(KeyCode::Right) => chain_cursor.add_x(),
                _ => panic!("what"),
            }
        }
    }

    // If a phrase exists at the new cursor location, set the currently edited chain to that.
    // Else, leave it at what it was before.
    // Note how we ignore the X value (whether its over a transpose or chain).
    // This is how LSDj works.
    if let Some(chain) = chains.get(edited_chain.0 as usize) {
        if let Some(phrase) = chain.get_phrase(chain_cursor.get_y() as usize) {
            edited_phrase.0 = phrase;
        }
    }
}

fn type_value(
    input: Res<InputRes>,
    chain_cursor: Res<cursors::ChainCursor>,
    mut chains: ResMut<types::chain::Chains>,
    edited_chain: ResMut<edited::EditedChain>,
) {
    for key in InputType::hex_keycodes() {
        if input.just_pressed(&key) {
            let (cursor_x, cursor_y) = chain_cursor.get_pos();
            let cursor_y = cursor_y as usize;

            if let Some(chain) = chains.get_mut(edited_chain.0 as usize) {
                if cursor_x == 0 {
                    // Editing a phrase value. Default to 0 if no phrase there.
                    let mut phrase = chain.get_phrase(cursor_y).unwrap_or(0);

                    // Move the second digit into the first digit.
                    // If they decide to type `ff` for an empty position,
                    // it's their fault lol (i mean what else could we do?)
                    phrase <<= 4;
                    // Add the pressed input to it
                    phrase += key.input_to_num().unwrap_or(0) as u8;

                    chain.set_phrase(cursor_y, phrase);
                } else {
                    // Editing a transpose value.
                    let mut transpose = chain
                        .get_transpose(cursor_y)
                        .expect("Transpose index out of bounds.");

                    // Move the second digit into the first digit.
                    transpose <<= 4;
                    // Add the pressed input to it
                    transpose += key.input_to_num().unwrap_or(0) as u8;

                    chain.set_transpose(cursor_y, transpose);
                }
            }
        }
    }
}

fn delete_value_system(
    input: Res<InputRes>,
    chain_cursor: Res<cursors::ChainCursor>,
    mut chains: ResMut<types::chain::Chains>,
    edited_chain: ResMut<edited::EditedChain>,
) {
    if let Some(chain) = chains.get_mut(edited_chain.0 as usize) {
        // If the `Delete` or `Backspace` keys are pressed,
        // delete the phrase or transpose at the songcursor position.
        for key in [
            InputType::Key(KeyCode::Delete),
            InputType::Key(KeyCode::Back),
        ] {
            if input.dr_pressed(&key) {
                let (cursor_x, cursor_y) = chain_cursor.get_pos();
                delete_value(cursor_x == 0, cursor_y, chain);
            }
        }

        // If the middle mouse button is clicked, delete the chain
        // where the mousecursor is hovering.
        if input.just_pressed(&InputType::Mouse(MouseButton::Middle)) {
            if let Some(cursor_pos) = input.get_cursor_tile_position() {
                if let Some((is_chain, index, _)) = hover(cursor_pos) {
                    delete_value(is_chain, index, chain);
                }
            }
        }
    }
}

/// Delete a value at this position.
fn delete_value(is_chain: bool, index: u8, chain: &mut types::chain::Chain) {
    if is_chain {
        chain.clear_phrase(index as usize);
    } else {
        chain.set_transpose(index as usize, 0x00);
    }
}

/// If a set chain is double-clicked
/// then open the chain screen on that value.
fn open_phrase_system(
    input: Res<InputRes>,
    mut chains: ResMut<types::chain::Chains>,
    mut state: ResMut<states::NextState>,
    edited_chain: ResMut<edited::EditedChain>,
) {
    // Move to the chain screen if an active cell is double-clicked.
    // So actually check for a double-click
    if !input.double_click() {
        return;
    }

    if let Some(cursor_pos) = input.get_cursor_tile_position() {
        if let Some((is_chain, index, _)) = hover(cursor_pos) {
            // Only do it if they double-click a chain (i.e. ignore double-clicking a transpose)
            if !is_chain {
                return;
            }

            if let Some(chain) = chains.get_mut(edited_chain.0 as usize) {
                let phrase = chain.get_phrase(index as usize);

                // Only move if the clicked phrase has contents
                if phrase.is_some() {
                    state.0 = Some((2, 1))
                }
            }
        }
    }
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
        let mut color = [colors::Colors::Background, colors::Colors::Highlight];

        // Draw the phrase
        // Set color if cursor is currently there.
        if chain_cursor_pos.1 == y as u8 {
            color[chain_cursor_pos.0 as usize] = colors::Colors::Cursor;
        };

        // Get the phrase and adjust its text content.
        let text = if let Some(phrase) = chain.get_phrase(y) {
            format!("{:02x}", phrase)
        } else {
            "--".to_string()
        };

        // Set the tiles.
        lh.set_tiles_string("map", 2, y + 2, &text, color[0])
            .unwrap();

        // Draw the transpose.
        lh.set_tiles_hex(
            "map",
            5,
            y + 2,
            chain.get_transpose(y).unwrap() as usize,
            2,
            color[1],
        )
        .unwrap();
    }
}
