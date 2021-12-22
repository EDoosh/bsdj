use super::*;
use crate::resources::song_cursor::*;
use crate::states;
use crate::tilerender::*;
use bevy::prelude::*;

pub struct NavBarPlugin;

impl Plugin for NavBarPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(states::States::Song).with_system(enter_scene.system()),
        );
        app.add_system_set(
            SystemSet::on_update(states::States::Song).with_system(draw_screen.system()),
        );
        // app.add_system_set(
        //     SystemSet::on_exit(states::States::Song).with_system(exit_game.system()),
        // );
    }
}

fn enter_scene(mut lh: ResMut<LayerHandler>) {}

fn draw_screen(song_cursor: Res<SongCursor>, mut lh: ResMut<LayerHandler>) {
    // Clear the 15th and 17th rows, leaving the 16th as it will be filled later.
    for x in 15..=19 {
        lh.set_tile("ui", x, 15, "space", colors::Colors::Highlight)
            .unwrap();
        lh.set_tile("ui", x, 17, "space", colors::Colors::Highlight)
            .unwrap();
    }

    let structure = ["ppwst", "scpit", "ggggg"];
    let col = 0;
    let row = 1;

    // Set the single tile on the top row
    lh.set_tile(
        "ui",
        15 + col,
        15,
        &structure[0][col..=col],
        colors::Colors::Details,
    )
    .unwrap();
    // Set the tiles in the middle row
    lh.set_tiles_string("ui", 15, 16, structure[1], colors::Colors::Details)
        .unwrap();
    // Set the single tile on the bottom row
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
        colors::Colors::Highlight,
    )
    .unwrap();
}
