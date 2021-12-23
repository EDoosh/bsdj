use crate::tilerender::*;
use crate::utils;
use bevy::prelude::*;

// Pixels per second
const SPEED: f32 = 5. * 8.;
const FREEZE_TIME_START: f32 = 3.;
const FREEZE_TIME_END: f32 = 5.;

pub struct HeadingTextPlugin;

impl Plugin for HeadingTextPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<HeadingTextEvent>();
        app.insert_resource(HeadingTextResource {
            active: false,
            length: 0,
            start_time: 0.,
        });

        app.add_system(read_event.system());
        app.add_system(move_text.system());
    }
}

pub struct HeadingTextEvent(pub String);

pub struct HeadingTextResource {
    active: bool,
    length: usize,
    start_time: f32,
}

fn read_event(
    mut reader: EventReader<HeadingTextEvent>,
    mut res: ResMut<HeadingTextResource>,
    mut lh: ResMut<LayerHandler>,
    time: Res<Time>,
) {
    for ev in reader.iter() {
        let t = ev.0.clone().to_lowercase();
        let mut text = Vec::new();

        for letter in t.split_terminator("").skip(1) {
            text.push(match letter {
                " " => "space",
                "*" => "copyright",
                "@" => "special",
                "^" => "musicnote",
                _ => letter,
            })
        }

        let text_length = std::cmp::max(text.len(), 20);
        lh.get_layer_mut("headingtext")
            .unwrap()
            .set_width(text_length);

        lh.clear_layer("headingtext", "space", colors::Colors::TextCursor)
            .unwrap();
        lh.set_tiles("headingtext", 0, 0, &text, colors::Colors::TextCursor)
            .unwrap();

        res.active = true;
        res.length = text_length;
        res.start_time = time.seconds_since_startup() as f32;
    }
}

fn move_text(mut res: ResMut<HeadingTextResource>, mut lh: ResMut<LayerHandler>, time: Res<Time>) {
    if !res.active {
        return;
    }

    let time_since_start = time.seconds_since_startup() as f32 - res.start_time;

    let time_from_start_to_kill =
        FREEZE_TIME_START + FREEZE_TIME_END + (8. * (res.length as f32 - 20.)) / SPEED;
    if time_since_start > time_from_start_to_kill {
        res.active = false;
        lh.clear_layer("headingtext", "space", colors::Colors::Empty)
            .unwrap();
        return;
    }

    let layer = lh.get_layer_mut("headingtext").unwrap();

    let x_shift = SPEED * (time_since_start - FREEZE_TIME_START);
    let max_x_shift = 8. * (res.length as f32 - 20.);
    let clamped_x_shift = utils::clampf(0., x_shift, max_x_shift);

    layer.set_position(Vec2::new(-clamped_x_shift, 0.));
}
