use super::*;
use crate::resources::{input::*, *};
use crate::states;
use crate::tilerender::*;
use crate::utils;
use bevy::prelude::*;

pub struct SongScene;

impl Plugin for SongScene {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(states::States::Song).with_system(enter_scene.system()),
        );
        app.add_system_set(
            SystemSet::on_update(states::States::Song)
                .with_system(update_scene.system())
                .with_system(draw_screen.system()),
        );
        // app.add_system_set(
        //     SystemSet::on_exit(states::States::Song).with_system(exit_game.system()),
        // );
    }
}

fn enter_scene(mut lh: ResMut<LayerHandler>) {
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

    // let all_tileids = tr.get_all_tile_ids().clone();
    // for (idx, tileid) in all_tileids.iter().enumerate() {
    //     lh.set_tile("map", idx % 20, idx / 20, tileid, "bg");
    // }
}

fn update_scene(
    mut commands: Commands,
    input: Res<InputRes>,
    mut song_cursor: ResMut<song_cursor::SongCursor>,
    mut channels: ResMut<types::channel::Channels>,
) {
    let cam = song_cursor.get_cam();
    let scroll_delta = input.get_scroll_delta();

    // Scroll wheel
    if scroll_delta != 0 {
        // Control key pressed: Change value of chain
        if input.exclusively_pressed(&[InputType::Key(KeyCode::LControl)]) {
            let cursor_pos = input.get_cursor_tile_position();

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
                    new = utils::clamp(0, new, 0x7f)
                }
                channel.set_chain(chain_y, new as u8);
            }
        } else {
            // Subtract so scrolling down causes cam to increase
            let new_cam = cam as isize - scroll_delta as isize;
            song_cursor.set_cam(new_cam);
        }
    }

    if input.just_pressed(&InputType::Mouse(MouseButton::Left)) {
        let cursor_pos = input.get_cursor_tile_position();

        if let Some((channel_index, chain_y, _)) = hover_on_chain(cursor_pos, cam) {
            song_cursor.set_x(channel_index as isize);
            song_cursor.set_y(chain_y as isize);
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
    song_cursor: Res<song_cursor::SongCursor>,
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
        for (i, channel) in channels.get().iter().enumerate() {
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

// fn exit_game(mut commands: Commands) {}
