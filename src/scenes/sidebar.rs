use crate::resources::*;
use crate::states;
use crate::tilerender::*;
use bevy::prelude::*;

pub struct SideBarPlugin;

impl Plugin for SideBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enter_scene).add_system(draw_screen);
    }
}

fn enter_scene(mut lh: ResMut<LayerHandler>, load_scene: ResMut<states::LoadState>) {
    // Only reload when it needs to be reloaded (i.e. a screen transition)
    if !load_scene.0 {
        return;
    }

    for i in 0..=9 {
        lh.set_tiles(
            "ui",
            17,
            i,
            &["space", "space", "space"],
            colors::Colors::Highlight,
        )
        .unwrap();
    }
    lh.set_tile("ui", 16, 4, "musicnote", colors::Colors::Background)
        .unwrap();
    lh.set_tile("ui", 16, 6, "1", colors::Colors::Background)
        .unwrap();
    lh.set_tile("ui", 16, 7, "2", colors::Colors::Background)
        .unwrap();
    lh.set_tile("ui", 16, 8, "w", colors::Colors::Background)
        .unwrap();
    lh.set_tile("ui", 16, 9, "n", colors::Colors::Background)
        .unwrap();
}

fn draw_screen(
    song_cursor: Res<cursors::SongCursor>,
    chain_cursor: Res<cursors::ChainCursor>,
    mut lh: ResMut<LayerHandler>,
) {
    lh.set_tiles_string(
        "ui",
        17,
        0,
        match song_cursor.get_x() {
            0 => "pu1",
            1 => "pu2",
            2 => "wav",
            3 => "noi",
            _ => panic!("Invalid Song Cursor X: {}", song_cursor.get_x()),
        },
        colors::Colors::Highlight,
    )
    .unwrap();

    lh.set_tiles_string(
        "ui",
        17,
        2,
        &format!("cp{:X}", chain_cursor.get_y()),
        colors::Colors::Highlight,
    )
    .unwrap();
}
