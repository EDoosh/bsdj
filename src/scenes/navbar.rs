use crate::resources::{cursors::NavCursor, edited::EditedInstrument, input::*};
use crate::states::*;
use crate::tilerender::*;
use bevy::prelude::*;

pub struct NavBarPlugin;

impl Plugin for NavBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(input_switch_scene);
        app.add_system(draw_screen);
        app.add_system_to_stage(CoreStage::PostUpdate, switch_scene);
    }
}

pub fn get_navbar_order(editing_instrument: &EditedInstrument) -> [[States; 5]; 3] {
    // Unless you want to manually change all references to
    // `state::NextState`, I would not recommend changing this.
    // You would also need to change NavCursor's min and max values.
    let mut structure = [
        [
            States::Project,
            States::File,
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
            States::Help,
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

/// Runs as part of PostUpdate so any scene changes in Update are registered
/// before the next frame begins.
// Allow so switching the state and not handling the error doesn't warn us.
#[allow(unused_must_use)]
fn switch_scene(
    mut new_state_pos: ResMut<NextState>,
    mut state: ResMut<State<States>>,
    mut load_scene: ResMut<LoadState>,
    mut nav_cursor: ResMut<NavCursor>,
    instrument: Res<EditedInstrument>,
) {
    // If a new state has been reloaded, reset it.
    if load_scene.0 {
        load_scene.0 = false;
    }

    // Take so next time this is called it doesn't try to
    // reload the same state unless it was actually set.
    if let Some(new_state_pos) = new_state_pos.0.take() {
        let structure = get_navbar_order(&*instrument);
        let new_state = structure[new_state_pos.1 as usize][new_state_pos.0 as usize];

        state.overwrite_replace(new_state);
        load_scene.0 = true;
        nav_cursor.set_x(new_state_pos.0 as isize);
        nav_cursor.set_y(new_state_pos.1 as isize);
    }
}

fn input_switch_scene(
    input: Res<InputRes>,
    mut state: ResMut<NextState>,
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
            state.0 = Some((relative_cursor_x as u8, relative_cursor_y as u8));
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

            let text = tile.abbr();

            lh.set_tiles_string(
                "ui",
                20 - column_count + tile_col_idx,
                18 - row_count + tile_row_idx,
                text,
                color,
            )
            .unwrap();
        }
    }
}
