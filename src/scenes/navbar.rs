use super::*;
use crate::resources::{edited_instrument::EditedInstrument, nav_cursor::NavCursor};
use crate::states;
use crate::tilerender::*;
use bevy::prelude::*;

pub struct NavBarPlugin;

impl Plugin for NavBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(states::States::Song).with_system(enter_scene));
        app.add_system_set(SystemSet::on_update(states::States::Song).with_system(draw_screen));
        // app.add_system_set(
        //     SystemSet::on_exit(states::States::Song).with_system(exit_game),
        // );
    }
}

fn enter_scene(mut lh: ResMut<LayerHandler>) {}

fn draw_screen(
    nav_cursor: Res<NavCursor>,
    mut lh: ResMut<LayerHandler>,
    instrument: Res<EditedInstrument>,
) {
    // Clear the 15th and 17th rows, leaving the 16th as it will be filled later.
    for x in 15..=19 {
        lh.set_tile("ui", x, 15, "space", colors::Colors::Highlight)
            .unwrap();
        lh.set_tile("ui", x, 17, "space", colors::Colors::Highlight)
            .unwrap();
    }

    let mut structure = ["ppwst", "scpit", "ggggg"];
    if instrument.0 == 40 {
        structure[1] = "scpiw"
    }
    let col = nav_cursor.get_x() as usize;
    let row = nav_cursor.get_y() as usize;

    // Set the tiles in the rows
    lh.set_tiles_string("ui", 15, 15, structure[0], colors::Colors::Highlight)
        .unwrap();
    lh.set_tiles_string("ui", 15, 16, structure[1], colors::Colors::Details)
        .unwrap();
    lh.set_tiles_string("ui", 15, 17, structure[2], colors::Colors::Highlight)
        .unwrap();

    lh.set_tile(
        "ui",
        15 + col,
        15,
        &structure[0][col..=col],
        colors::Colors::Details,
    )
    .unwrap();
    lh.set_tile(
        "ui",
        15 + col,
        17,
        &structure[2][col..=col],
        colors::Colors::Details,
    )
    .unwrap();

    // Set the tile corresponding to the screen the user is currently on.
    lh.set_tile(
        "ui",
        15 + col,
        15 + row,
        &structure[row][col..=col],
        colors::Colors::Cursor,
    )
    .unwrap();
}
