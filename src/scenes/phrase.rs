use crate::resources::{input::*, *};
use crate::states;
use crate::tilerender::*;
use bevy::prelude::*;

pub struct PhraseScene;

impl Plugin for PhraseScene {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(states::States::Phrase)
                .with_system(enter_scene)
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

fn draw_screen(
    mut lh: ResMut<LayerHandler>,
    song_cursor: Res<cursors::SongCursor>,
    phrase_cursor: Res<cursors::PhraseCursor>,
    phrases: Res<types::Phrases>,
    edited_phrase: Res<edited::EditedPhrase>,
    instruments: Res<types::Instruments>,
) {
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
    let phrase_cursor_pos = phrase_cursor.get_pos();

    // If kit is true, the rows take a different shape.
    // The only thing that can cause kit to change is the instrument.
    // If the previous instrument was a kit, the following rows will also
    // look like a kit row until an instrument that isnt a kit is reached.
    let mut kit = false;
    for y in 0..16 {
        let note = phrase.get_note(y);
        let instr_id = phrase.get_instr(y);
        let cmd = phrase.get_cmd(y);
        let cmd_val = phrase
            .get_cmd_val(y)
            .expect("Expected a command value in phrase, got nothing.");

        let mut instr = None;
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
            kit = temp_instr.is_kit();
        }

        if !kit {
            let mut tile_colors = [
                Colors::Background,
                Colors::Background,
                Colors::Highlight,
                Colors::Background,
            ];
            if phrase_cursor_pos.1 as usize == y {
                tile_colors[phrase_cursor_pos.0 as usize] = Colors::Cursor;
            }

            let mut note_text = "---".to_string();
            if let Some(note) = note {
                // Write a note using the speech instrument.
                if let Some(types::instrument::Instrument::Speech()) = instr {
                    todo!("Print the name of the speech instrument.");
                } else {
                    // Subtract 1 as the first Note value will be 1,
                    // however 0 should be C3
                    let note_str = match (note - 1) % 12 {
                        0 => "C",
                        1 => "C#",
                        2 => "D",
                        3 => "D#",
                        4 => "E",
                        5 => "F",
                        6 => "F#",
                        7 => "G",
                        8 => "G#",
                        9 => "A",
                        10 => "A#",
                        11 => "B",
                        _ => panic!("note % 12 greater than 11?"),
                    };
                    let mut octave = (note - 1) / 12;
                    // In Pu1, Pu2, and Noi, octaves display 3-B, so add 3 to the octave.
                    // But in a wave, they display 2-A, so add 2.
                    octave += if song_cursor.is_wav() { 2 } else { 3 };

                    note_text = format!("{:2}{:X}", note_str, octave)
                }
            }

            lh.set_tiles_string("map", 2, y + 2, &note_text, tile_colors[0])
                .unwrap();

            lh.set_tiles_string("map", 6, y + 2, "i", Colors::Highlight)
                .unwrap();
            if let Some(instr_id) = instr_id {
                lh.set_tiles_hex("map", 7, y + 2, instr_id as usize, 2, tile_colors[1])
                    .unwrap();
            } else {
                lh.set_tiles_string("map", 7, y + 2, "--", tile_colors[1])
                    .unwrap();
            }

            let cmd = cmd.unwrap_or(types::command::Command::None);
            lh.set_tiles_string(
                "map",
                10,
                y + 2,
                &cmd.get_abbr().unwrap_or('-').to_string(),
                tile_colors[2],
            )
            .unwrap();
            lh.set_tiles_string("map", 11, y + 2, &cmd.get_val_str(cmd_val), tile_colors[3])
                .unwrap();
        } else {
            todo!("Kit representation.");
        }
    }
}
