use crate::events::HeadingTextEvent;
use crate::resources::{input::*, *};
use crate::states;
use crate::tilerender::*;
use bevy::prelude::*;

pub struct SongScene;

impl Plugin for SongScene {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(states::States::Song)
                .with_system(enter_scene)
                .with_system(handle_scroll)
                .with_system(move_cursor)
                .with_system(type_chain)
                .with_system(delete_chain_system)
                .with_system(bookmark_chain_system)
                .with_system(open_chain_system)
                .with_system(draw_screen),
        );
        // app.add_system_set(
        //     SystemSet::on_exit(states::States::Song).with_system(exit_game),
        // );
    }
}

fn enter_scene(mut lh: ResMut<LayerHandler>, load_scene: ResMut<states::LoadState>) {
    // Dont try enter the scene if the scene should not be loaded.
    if !load_scene.0 {
        return;
    }

    lh.clear_layer("map", "space", colors::Colors::Background)
        .unwrap();
    lh.set_tiles_string("map", 0, 0, "song", colors::Colors::Background)
        .unwrap();
    lh.set_tiles(
        "map",
        3,
        1,
        &[
            "pu", "pu1", "", "pu", "pu2", "", "wav1", "wav2", "", "noi1", "noi2",
        ],
        colors::Colors::Background,
    )
    .unwrap();
}

fn handle_scroll(
    input: Res<InputRes>,
    mut song_cursor: ResMut<cursors::SongCursor>,
    mut channels: ResMut<types::channel::Channels>,
) {
    let cam = song_cursor.get_cam();
    let scroll_delta = input.get_scroll_delta();

    // Scroll wheel
    if scroll_delta == 0 {
        return;
    }

    // Control key pressed: Change value of chain
    if input.exclusively_pressed(&[InputType::Key(KeyCode::LControl)]) {
        if let Some(cursor_pos) = input.get_cursor_tile_position() {
            if let Some((channel_index, chain_y, left)) = hover_on_chain(cursor_pos, cam) {
                // Get the chain info
                let channel = channels.get_mut(channel_index as usize);
                let current = channel.get_chain(chain_y);

                let change = if left { 0x10 } else { 0x01 };

                // If there is no chain there already, set to 0.
                // Else add the change, but clamp between 0 and 0x7f
                let mut new = 0;
                if let Some(chain) = current {
                    new = chain as i32 + change * scroll_delta;
                    new = new.clamp(0x00, 0x7f)
                }

                channel.set_chain(chain_y, new as u8);
            }
        }
    } else {
        // Subtract so scrolling down causes cam to increase
        let new_cam = cam as isize - scroll_delta as isize;
        song_cursor.set_cam(new_cam);
    }
}

fn move_cursor(
    input: Res<InputRes>,
    mut song_cursor: ResMut<cursors::SongCursor>,
    channels: Res<types::channel::Channels>,
    mut edited_chain: ResMut<edited::EditedChain>,
) {
    let cam = song_cursor.get_cam();

    if input.just_pressed(&InputType::Mouse(MouseButton::Left)) {
        if let Some(cursor_pos) = input.get_cursor_tile_position() {
            if let Some((channel_index, chain_y, _)) = hover_on_chain(cursor_pos, cam) {
                song_cursor.set_x(channel_index as isize);
                song_cursor.set_y(chain_y as isize);
            }
        }
    }

    for key in InputType::directional_keycodes() {
        if input.dr_pressed(&key) && input.exclusively_pressed(&[key]) {
            match key {
                InputType::Key(KeyCode::Up) => song_cursor.sub_y(),
                InputType::Key(KeyCode::Down) => song_cursor.add_y(),
                InputType::Key(KeyCode::Left) => song_cursor.sub_x(),
                InputType::Key(KeyCode::Right) => song_cursor.add_x(),
                _ => panic!("what"),
            }
        }
    }

    let channel = channels.get(song_cursor.get_x() as usize);
    // If a chain exists at the new cursor location, set the currently edited chain to that.
    // Else, leave it at what it was before.
    if let Some(chain) = channel.get_chain(song_cursor.get_y()) {
        edited_chain.0 = chain;
    }
}

fn type_chain(
    input: Res<InputRes>,
    song_cursor: Res<cursors::SongCursor>,
    mut channels: ResMut<types::channel::Channels>,
) {
    for key in InputType::hex_keycodes() {
        if input.just_pressed(&key) {
            let (cursor_x, cursor_y) = song_cursor.get_pos();

            let channel = channels.get_mut(cursor_x as usize);
            let mut chain = channel.get_chain(cursor_y).unwrap_or(0);

            // Only get the second digit but move it up to the first.
            // if the value is greater than 7, it would lead to the chain
            // number being too large, mod by 8 instead of 16.
            chain = chain % 0x08 * 0x10;
            // Add the pressed input to it
            chain += key.input_to_num().unwrap_or(0) as u8;

            channel.set_chain(cursor_y, chain);
        }
    }
}

fn delete_chain_system(
    input: Res<InputRes>,
    song_cursor: Res<cursors::SongCursor>,
    mut channels: ResMut<types::channel::Channels>,
) {
    // If the `Delete` or `Backspace` keys are pressed,
    // delete the chain at the songcursor position.
    for key in [
        InputType::Key(KeyCode::Delete),
        InputType::Key(KeyCode::Back),
    ] {
        if input.dr_pressed(&key) {
            let (cursor_x, cursor_y) = song_cursor.get_pos();
            delete_chain(cursor_x as usize, cursor_y, &mut channels);
        }
    }

    // If the middle mouse button is clicked, delete the chain
    // where the mousecursor is hovering.
    if input.just_pressed(&InputType::Mouse(MouseButton::Middle)) {
        if let Some(cursor_pos) = input.get_cursor_tile_position() {
            let cam = song_cursor.get_cam();

            if let Some((channel_index, chain_y, _)) = hover_on_chain(cursor_pos, cam) {
                delete_chain(channel_index, chain_y, &mut channels);
            }
        }
    }
}

/// Delete or remove a chain at this position.
fn delete_chain(channel_index: usize, chain_y: u8, channels: &mut types::channel::Channels) {
    let channel = channels.get_mut(channel_index as usize);
    let chain = channel.get_chain(chain_y);

    if chain.is_some() {
        // Just remove the value of the chain
        channel.clear_chain(chain_y);
    } else {
        // Chain is already empty.
        // Move all cells below it up.
        channel.remove_chain_slot(chain_y);
    }
}

fn bookmark_chain_system(
    input: Res<InputRes>,
    song_cursor: Res<cursors::SongCursor>,
    mut channels: ResMut<types::channel::Channels>,
    mut headtext_writer: EventWriter<HeadingTextEvent>,
) {
    // If the `M` key is pressed, bookmark the tile the songcursor is on.
    for key in [InputType::Key(KeyCode::M)] {
        if input.just_pressed(&key) {
            let (cursor_x, cursor_y) = song_cursor.get_pos();

            if let Err(e) = bookmark_chain(cursor_x as usize, cursor_y, &mut channels) {
                headtext_writer.send(HeadingTextEvent(e.to_string()));
            }
        }
    }

    // Bookmark the tile the mousecursor is hovering over if the
    // right mouse button is clicked.
    if input.just_pressed(&InputType::Mouse(MouseButton::Right)) {
        if let Some(cursor_pos) = input.get_cursor_tile_position() {
            let cam = song_cursor.get_cam();
            if let Some((channel_index, chain_y, _)) = hover_on_chain(cursor_pos, cam) {
                let err = bookmark_chain(channel_index, chain_y, &mut channels);

                if let Err(e) = err {
                    headtext_writer.send(HeadingTextEvent(e.to_string()));
                }
            }
        }
    }
}

/// Toggle a bookmark at this position.
fn bookmark_chain(
    channel_index: usize,
    chain_y: u8,
    channels: &mut types::channel::Channels,
) -> Result<bool, &str> {
    let channel = channels.get_mut(channel_index as usize);
    channel.toggle_bookmark(chain_y)
}

/// If a set chain is double-clicked
/// then open the chain screen on that value.
fn open_chain_system(
    input: Res<InputRes>,
    song_cursor: Res<cursors::SongCursor>,
    mut channels: ResMut<types::channel::Channels>,
    mut state: ResMut<states::NextState>,
) {
    // Move to the chain screen if an active cell is double-clicked.
    if !input.double_click() {
        return;
    }

    if let Some(cursor_pos) = input.get_cursor_tile_position() {
        let cam = song_cursor.get_cam();

        if let Some((channel_index, chain_y, _)) = hover_on_chain(cursor_pos, cam) {
            let channel = channels.get_mut(channel_index as usize);
            let chain = channel.get_chain(chain_y);

            // Only move if the clicked chain has contents
            if chain.is_some() {
                state.0 = Some((1, 1))
            }
        }
    }
}

/// Determines where the user cursor is on a chain.
/// Returns an Option. None means no chain is hovered.
/// The first parameter of a Some will be the channel hovered.
/// The second parameter will be the chain y-value hovered.
/// The third paramter will indicate whether the left value was hovered.
fn hover_on_chain(cursor_pos: (i32, i32), cam: u8) -> Option<(usize, u8, bool)> {
    let (x, y) = cursor_pos;
    if y < 2 {
        return None;
    }

    let left;
    if [3, 6, 9, 12].contains(&x) {
        // Left column
        left = true;
    } else if [4, 7, 10, 13].contains(&x) {
        // Right column
        left = false;
    } else {
        // No chain column
        return None;
    }

    let channel_index = x / 3 - 1;
    let chain_y = cam + (y as u8 - 2);
    Some((channel_index as usize, chain_y, left))
}

fn draw_screen(
    song_cursor: Res<cursors::SongCursor>,
    mut lh: ResMut<LayerHandler>,
    channels: Res<types::channel::Channels>,
) {
    let cam_pos = song_cursor.get_cam() as usize;
    // For the 16 columns
    for y in 0..16 {
        let y_cam = (y + cam_pos) as u8;
        // Left index bar
        lh.set_tiles_hex("map", 0, y + 2, y_cam as usize, 2, colors::Colors::Details)
            .unwrap();

        // For each channel column
        for (i, channel) in channels.get_all().iter().enumerate() {
            // Default text and color
            let mut text = "--".to_string();
            let mut color = colors::Colors::Background;

            // Get the chain and if it's bookmarked and adjust accordingly
            if let Some(chain) = channel.get_chain(y_cam) {
                text = format!("{:02x}", chain);
            }
            if channel.is_bookmarked(y_cam) {
                color = colors::Colors::Highlight;
            }
            if song_cursor.get_pos() == (i as u8, y_cam) {
                color = colors::Colors::Cursor;
            }

            // Set the tiles.
            lh.set_tiles_string("map", 3 + i * 3, y + 2, &text, color)
                .unwrap();
        }
    }
}
