use super::*;
use crate::resources::{edited_instrument::EditedInstrument, input::*, nav_cursor::NavCursor};
use crate::states::States;
use crate::tilerender::*;
use bevy::prelude::*;

pub struct NavBarPlugin;

impl Plugin for NavBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(switch_scene);
        app.add_system(draw_screen);
        // app.add_system_set(SystemSet::on_enter(States::Song).with_system(enter_scene));
        // app.add_system_set(
        //     SystemSet::on_update(States::Song)
        //         .with_system(switch_scene)
        //         .with_system(draw_screen),
        // );
        // app.add_system_set(
        //     SystemSet::on_exit(states::States::Song).with_system(exit_game),
        // );
    }
}

pub fn get_navbar_order(editing_instrument: &EditedInstrument) -> [[States; 5]; 3] {
    let mut structure = [
        [
            States::Project,
            States::Project,
            States::Wave,
            States::Synth,
            States::Table,
        ],
        [
            States::Song,
            States::Chain,
            States::Phrase,
            States::Instrument,
            States::Table,
        ],
        [
            States::Groove,
            States::Groove,
            States::Groove,
            States::Groove,
            States::Groove,
        ],
    ];

    if editing_instrument.0 == 40 {
        structure[1][3] = States::Speech;
        structure[1][4] = States::Word;
    }

    structure
}

fn enter_scene(mut lh: ResMut<LayerHandler>) {}

fn switch_scene(
    input: Res<InputRes>,
    mut state: ResMut<State<States>>,
    mut nav_cursor: ResMut<NavCursor>,
    instrument: Res<EditedInstrument>,
) {
    if !input.just_pressed(&InputType::Mouse(MouseButton::Left)) {
        return;
    }

    if let Some(cursor_pos) = input.get_cursor_tile_position() {
        let structure = get_navbar_order(&*instrument);

        // Relative to the top-left of the navbar.
        let relative_cursor_x = cursor_pos.0 as isize - (20 - structure[0].len() as isize);
        let relative_cursor_y = cursor_pos.1 as isize - (18 - structure.len() as isize);

        // Clicked within the navbar.
        if relative_cursor_x >= 0 && relative_cursor_y >= 0 {
            nav_cursor.set_x(relative_cursor_x);
            nav_cursor.set_y(relative_cursor_y);

            state
                .overwrite_replace(
                    structure[relative_cursor_y as usize][relative_cursor_x as usize],
                )
                .unwrap()
        }
    }
}

fn draw_screen(
    nav_cursor: Res<NavCursor>,
    mut lh: ResMut<LayerHandler>,
    instrument: Res<EditedInstrument>,
) {
    let structure = get_navbar_order(&*instrument);
    let cursor_col = nav_cursor.get_x() as usize;
    let cursor_row = nav_cursor.get_y() as usize;

    let row_count = structure.len();
    let column_count = structure[1].len();

    for (tile_row_idx, tile_row) in structure.iter().enumerate() {
        for (tile_col_idx, tile) in tile_row.iter().enumerate() {
            let mut color = colors::Colors::Highlight;

            // Middle tile row and the tiles on the same column as the
            // selected scene should be a different colour.
            if tile_row_idx == 1 || tile_col_idx == cursor_col {
                color = colors::Colors::Details;
            }

            // The cursor should be its own colour.
            if tile_row_idx == cursor_row && tile_col_idx == cursor_col {
                color = colors::Colors::Cursor;
            }

            lh.set_tiles_string(
                "ui",
                20 - column_count + tile_col_idx,
                18 - row_count + tile_row_idx,
                tile.abbr(),
                color,
            )
            .unwrap();
        }
    }
}
