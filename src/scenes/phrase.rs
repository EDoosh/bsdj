use crate::resources::{cursors::phrase, input::*, *};
use crate::states;
use crate::tilerender::*;
use crate::utils::u8_utils::WrappingAdd;
use bevy::prelude::*;

pub struct PhraseScene;

impl Plugin for PhraseScene {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(states::States::Phrase)
                .with_system(enter_scene)
                .with_system(set_edited_to_row)
                .with_system(handle_scroll)
                .with_system(move_cursor)
                .with_system(draw_screen),
        );
    }
}

fn enter_scene(
    mut lh: ResMut<LayerHandler>,
    edited_phrase: Res<edited::EditedPhrase>,
    load_scene: ResMut<states::LoadState>,
) {
    // Dont try enter the scene if the scene should not be loaded.
    if !load_scene.0 {
        return;
    }

    // Clear the map
    lh.clear_layer("map", "space", Colors::Background).unwrap();

    // Set the top of the screen to say "PHRASE XX"
    lh.set_tiles_string(
        "map",
        0,
        0,
        &format!("phrase {:02x}", edited_phrase.0),
        Colors::Background,
    )
    .unwrap();

    // Set the tiles underneath to say "NOTE", "INSTR", and "CMD"
    lh.set_tiles(
        "map",
        2,
        1,
        &[
            "note1", "note2", "note3", "", "instr1", "instr2", "instr3", "", "cmd1", "cmd2",
        ],
        colors::Colors::Background,
    )
    .unwrap();

    // Write the phrase indexes on the side
    for y in 0..16 {
        lh.set_tiles_hex("map", 0, 2 + y, y, 1, colors::Colors::Details)
            .unwrap();
    }
}

/// If an instrument, groove cmd, table cmd, or word exists at the
/// new cursor location, set the currently edited chain to that.
/// Else, leave it at what it was before.
#[allow(clippy::too_many_arguments)]
fn set_edited_to_row(
    phrase_cursor: Res<cursors::PhraseCursor>,
    edited_phrase: Res<edited::EditedPhrase>,
    phrases: Res<types::Phrases>,
    instruments: Res<types::Instruments>,
    mut edited_instr: ResMut<edited::EditedInstrument>,
    mut edited_table: ResMut<edited::EditedTable>,
    mut edited_groove: ResMut<edited::EditedGroove>,
    mut edited_word: ResMut<edited::EditedWord>,
) {
    let x = phrase_cursor.get_column();
    let y = phrase_cursor.get_y();

    let phrase = phrases.get(edited_phrase.0 as usize);
    if phrase.is_none() {
        return;
    }
    let phrase = phrase.unwrap();

    // Set the edited instrument if there is an instrument on this line or before it.
    if let Some(instr_id) = note_index_instr(phrase, y) {
        edited_instr.0 = instr_id;
    }

    // Set the edited word if its on a speech instrument and there is a word there.
    if note_index_is_speech(phrase, &instruments, y) {
        if let Some(note) = phrase.get_note(y as usize) {
            edited_word.0 = note.get().unwrap();
        }
    }

    // Set the edited table or groove if on one of those commands.
    if x == phrase::PhraseCursorColumn::Command || x == phrase::PhraseCursorColumn::CommandValue {
        if let Some(cmd) = phrase.get_cmd(y as usize) {
            let cmdval = phrase.get_cmd_val(y as usize).unwrap();

            match cmd {
                // Only if cmdval is a valid table.
                types::Command::Table if cmdval <= 0x1f => {
                    edited_table.0 = cmdval;
                }
                // Only if cmdval is a valid groove.
                types::Command::Groove if cmdval <= 0x1f => {
                    edited_groove.0 = cmdval;
                }
                // Do nothing in the other circumstances.
                _ => {}
            }
        }
    }
}

fn handle_scroll(
    input: Res<InputRes>,
    edited_phrase: Res<edited::EditedPhrase>,
    mut phrases: ResMut<types::phrase::Phrases>,
) {
    let scroll_delta = input.get_scroll_delta();

    // Check they actually scrolled this frame.
    if scroll_delta == 0 {
        return;
    }

    // Require control key to be pressed to change value of anything.
    if !input.exclusively_pressed(&[InputType::Key(KeyCode::LControl)]) {
        return;
    }

    // Get the position of the cursor.
    let cursor_pos = input.get_cursor_tile_position();
    if cursor_pos.is_none() {
        return;
    }
    let cursor_pos = cursor_pos.unwrap();

    // Get the information about the tiles being hovered.
    let hover_data = hover(cursor_pos);
    if hover_data.is_none() {
        return;
    }
    let (column, index, is_left) = hover_data.unwrap();

    // Get the phrase info
    let phrase = phrases.get_mut(edited_phrase.0 as usize);
    if phrase.is_none() {
        return;
    }
    let phrase = phrase.unwrap();

    match column {
        phrase::PhraseCursorColumn::Note => {
            // Scrolling on the note.

            // If scrolling on the left side, increase/decrease by one note.
            // If scrolling on the right side, increase/decrease by one octave.
            let change = if is_left { 1 } else { 12 };

            // If there is no note there already, set to 1.
            // Else add the change, but clamp between 1 and phrase::LARGEST_NOTE
            let mut new = 1;
            if let Some(note) = phrase.get_note(index as usize) {
                // As get_note() is valid, get() on the note is always Some.
                new = note.get().unwrap() as i32 + change * scroll_delta;
                new = new.clamp(1, types::note::LARGEST_NOTE as i32);
            }
            phrase.set_note(index as usize, new as u8);
        }
        phrase::PhraseCursorColumn::Instrument => {
            // Scrolling on the instrument.

            // To match the displayed positions of what is changed...
            // If scrolling on the left side, increase/decrease by 0x10.
            // If scrolling on the right side, increase/decrease by 1.
            let change = if is_left { 0x10 } else { 1 };

            // If there is no command there already, set to 0.
            // Else add the change, but clamp between 0 and instrument::INSTR_COUNT - 1
            // The -1 accounts for 0-indexing.
            let mut new = 0;
            if let Some(instr) = phrase.get_instr(index as usize) {
                new = instr as i32 + change * scroll_delta;
                new = new.clamp(0, types::instrument::INSTR_COUNT as i32 - 1);
            }
            phrase.set_instr(index as usize, new as u8);
        }
        phrase::PhraseCursorColumn::Command => {
            // Scrolling on the command.

            // Always increase or decrease by 1.
            let change = 1;

            // If there is no command there already, set to 1.
            // Else add the change, but clamp between 1 and command::COMMAND_COUNT
            // Start at 1 as 0 is no command.
            let mut new = 1;
            if let Some(cmd) = phrase.get_cmd(index as usize) {
                new = cmd.to_num() as i32 + change * scroll_delta;
                new = new.clamp(1, types::command::COMMAND_COUNT as i32);
            }
            phrase.set_cmd(
                index as usize,
                types::command::Command::from_num(new as u8).unwrap(),
            );
        }
        phrase::PhraseCursorColumn::CommandValue => {
            // Scrolling on the command value.

            // To match the displayed positions of what is changed...
            // If scrolling on the left side, increase/decrease by 0x10.
            // If scrolling on the right side, increase/decrease by 1.
            let change = if is_left { 0x10 } else { 1 };
            let change = (change * scroll_delta) as isize;

            let mut cmdval = phrase.get_cmd_val(index as usize).unwrap();

            // By unwrapping into Command::None instead of using an if let Some(),
            // we can change the command value on a row where there is no command.
            let cmd = phrase
                .get_cmd(index as usize)
                .unwrap_or(types::Command::None);

            // Some commands have special requirements.
            match cmd {
                types::Command::Table | types::Command::Groove => {
                    // Tables and Grooves must be between 0 and 1f.
                    cmdval = (cmdval as isize + change).clamp(0, 0x1f) as u8;
                }
                types::Command::Pan => {
                    // If is_left is set, set L on or off depending on the scroll.
                    // If is_left is not set, set R on or off depending on the scroll.
                    if scroll_delta < 0 {
                        // Toggle off if scrolling down.
                        cmdval &= if is_left { 0b10 } else { 0b01 };
                    } else {
                        // Toggle on if scrolling up.
                        cmdval |= if is_left { 0b01 } else { 0b10 };
                    }
                }
                types::Command::Wave => {
                    // Add or remove the scroll_delta, clamping between 0 and 3.
                    cmdval = (cmdval as i32 + scroll_delta).clamp(0, 3) as u8;
                }
                _ => {
                    // Add to the command value, overflowing/underflowing if necessary.
                    // Safe to unwrap as index should never be greater than the command value length.
                    cmdval = cmdval.w_add(change);
                }
            }

            phrase.set_cmd_val(index as usize, cmdval);
        }
    };
}

fn move_cursor(input: Res<InputRes>, mut phrase_cursor: ResMut<cursors::PhraseCursor>) {
    // Move the cursor to the point the user clicked.
    if input.just_pressed(&InputType::Mouse(MouseButton::Left)) {
        if let Some(cursor_pos) = input.get_cursor_tile_position() {
            if let Some((column, index, _)) = hover(cursor_pos) {
                phrase_cursor.set_x(column.to_num() as isize);
                phrase_cursor.set_y(index as isize);
            }
        }
    }

    // Move the cursor based on directional inputs (Up, Left, Down, and Right)
    if let Some(key) = input.directional_input() {
        key.move_cursor(&mut *phrase_cursor);
    }
}

/// Determines where the user cursor is on an inputtable value.
/// Returns an Option. None means no inputtable value is hovered.
///
/// DOES NOT YET WORK FOR KITS.
///
/// The first parameter of the tuple will be the column hovered.
/// The second parameter will be the index hovered.
/// The third paramter will indicate whether the left value was hovered. Always false on Command.
fn hover(cursor_pos: (i32, i32)) -> Option<(phrase::PhraseCursorColumn, u8, bool)> {
    let (x, y) = cursor_pos;

    // Not high enough to click on a note row.
    if y < 2 {
        return None;
    }

    // Find the column and whether it was a left value.
    let (column, left) = match x {
        2 | 3 | 4 => (phrase::PhraseCursorColumn::Note, x != 4),
        7 | 8 => (phrase::PhraseCursorColumn::Instrument, x == 7),
        10 => (phrase::PhraseCursorColumn::Command, false),
        11 | 12 => (phrase::PhraseCursorColumn::CommandValue, x == 11),
        // If it was none of the above, they aren't hovering over an input.
        _ => return None,
    };

    // The row index the user was hovering over.
    let index = y as u8 - 2;
    Some((column, index, left))
}

/// Returns what instrument ID owns a row, or None if there are no instruments before it.
fn note_index_instr(phrase: &types::phrase::Phrase, index: u8) -> Option<u8> {
    assert!(
        index < 16,
        "Note index invalid: Expected 0-15, got {}",
        index
    );

    // Iterate backwards until an instrument is found.
    for i in index..=0 {
        let instr_id = phrase.get_instr(i as usize);
        if instr_id.is_some() {
            return instr_id;
        }
    }

    // Return None if no instrument is found.
    None
}

/// Returns whether the row specified is a kit or not.
///
/// If true, the rows take a different shape.
/// The only thing that can cause kit to change is the instrument.
/// If the previous set instrument was a kit, the following rows will also
/// look like a kit row until an instrument that isnt a kit is reached.
fn note_index_is_kit(
    phrase: &types::phrase::Phrase,
    instruments: &types::Instruments,
    index: u8,
) -> bool {
    if let Some(instr_id) = note_index_instr(phrase, index) {
        if let Some(instr) = instruments.get(instr_id as usize) {
            return instr.is_kit();
        }
    }
    false
}

/// Returns whether the row specified is a speech instrument or not.
fn note_index_is_speech(
    phrase: &types::phrase::Phrase,
    instruments: &types::Instruments,
    index: u8,
) -> bool {
    if let Some(instr_id) = note_index_instr(phrase, index) {
        if let Some(instr) = instruments.get(instr_id as usize) {
            return instr.is_speech();
        }
    }
    false
}

fn draw_screen(
    mut lh: ResMut<LayerHandler>,
    song_cursor: Res<cursors::SongCursor>,
    phrase_cursor: Res<cursors::PhraseCursor>,
    phrases: Res<types::Phrases>,
    edited_phrase: Res<edited::EditedPhrase>,
    instruments: Res<types::Instruments>,
) {
    // Get the phrase currently being worked on.
    let phrase = phrases.get(edited_phrase.0 as usize);

    // Show an error if the phrase doesn't exist.
    if phrase.is_none() {
        lh.set_tiles_string("map", 4, 4, "----------", Colors::Highlight)
            .unwrap();
        lh.set_tiles_string("map", 4, 5, "  error:  ", Colors::Highlight)
            .unwrap();
        lh.set_tiles_string("map", 4, 6, " invalid  ", Colors::Highlight)
            .unwrap();
        lh.set_tiles_string("map", 4, 7, "phrase id ", Colors::Highlight)
            .unwrap();
        lh.set_tiles_string("map", 4, 8, "----------", Colors::Highlight)
            .unwrap();
        return;
    }

    let phrase = phrase.unwrap();
    // Get the position of the cursor so we can render the cursor bg color somewhere.
    let phrase_cursor_pos = phrase_cursor.get_pos();

    // See `note_index_is_kit` for why we store this.
    // This does the same as that but is more effiecient as it stores
    // the previous result in this variable and calculates it before
    // displaying on screen, as we're getting the instrument anyways.
    let mut instr = None;

    // For all 16 note rows...
    for y in 0..16 {
        // Get info about the row.
        let note = phrase.get_note(y);
        let instr_id = phrase.get_instr(y);
        let cmd = phrase.get_cmd(y);
        let cmd_val = phrase
            .get_cmd_val(y)
            .expect("Expected a command value in phrase, got nothing.");

        // Get the instrument on this row and store in `instr`
        if let Some(instr_id) = instr_id {
            // Needs to be placed in a Some but also used to check if a kit,
            // so create this temp variable
            let temp_instr = instruments.get(instr_id as usize).unwrap_or_else(|| {
                panic!(
                    "Expected an instrument with ID {} but none was found.",
                    instr_id
                )
            });

            instr = Some(temp_instr);
        }

        if instr.is_some() && instr.unwrap().is_kit() {
            // Display kit.
            todo!("Kit representation.");
        } else {
            // The tile colours of each column.
            let mut tile_colors = [
                Colors::Background,
                Colors::Background,
                Colors::Highlight,
                Colors::Background,
            ];
            // Change the tile colour if it's where the cursor is.
            if phrase_cursor_pos.1 as usize == y {
                tile_colors[phrase_cursor_pos.0 as usize] = Colors::Cursor;
            }

            // If there are no notes at this position, by default show a '---'
            let mut note_text = "---".to_string();
            // But if there is a note, get the correct representation of it here.
            if let Some(note) = note {
                if let Some(types::instrument::Instrument::Speech()) = instr {
                    // Write a note using the speech instrument.
                    todo!("Print the name of the speech instrument.");
                } else {
                    // Unwrap as note cannot be empty.
                    note_text = note.to_string(song_cursor.is_wav()).unwrap();
                }
            }

            // Set note text.
            lh.set_tiles_string("map", 2, y + 2, &note_text, tile_colors[0])
                .unwrap();

            // Set instrument text and ID
            lh.set_tiles_string("map", 6, y + 2, "i", Colors::Highlight)
                .unwrap();
            if let Some(instr_id) = instr_id {
                lh.set_tiles_hex("map", 7, y + 2, instr_id as usize, 2, tile_colors[1])
                    .unwrap();
            } else {
                lh.set_tiles_string("map", 7, y + 2, "--", tile_colors[1])
                    .unwrap();
            }

            // Set command abbreviation and value
            let cmd = cmd.unwrap_or(types::command::Command::None);
            lh.set_tiles_string(
                "map",
                10,
                y + 2,
                &cmd.get_abbr().unwrap_or('-').to_string(),
                tile_colors[2],
            )
            .unwrap();
            lh.set_tiles("map", 11, y + 2, &cmd.get_val_str(cmd_val), tile_colors[3])
                .unwrap();
        }
    }
}
