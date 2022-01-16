use crate::tilerender::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

const FONTS: &[&str] = &["lowr"];
const GLYPHS: &[&str] = &["dflt"];
const COLORSETS: &[&str] = &["ami ", "crt ", "cute", "gray", "red ", "rgb "];

pub struct InitRendererPlugin;

impl Plugin for InitRendererPlugin {
    fn build(&self, app: &mut App) {
        let tr = TileRenderer::new(8, 8);
        let mut lh = LayerHandler::new(tr);
        init_fonts(&mut lh);
        init_glyphs(&mut lh);
        init_colorsets(&mut lh);

        construct_layers(&mut lh);
        app.insert_resource(lh);

        app.add_startup_system(spawn_renderer);
        #[cfg(debug_assertions)]
        app.add_system(fps_counter);
    }
}

fn init_fonts(lh: &mut LayerHandler) {
    for font in FONTS {
        let tr = lh.get_renderer_mut();
        let filename = format!("assets/fonts/{}.tilesprite", font);
        parse_tilesprite::TileSpriteParser::parse_and_add(&filename, font, tr).unwrap();

        lh.font_names.push(font.to_string());
    }
}

fn init_glyphs(lh: &mut LayerHandler) {
    for glyph in GLYPHS {
        let tr = lh.get_renderer_mut();
        let filename = format!("assets/glyphs/{}.tilesprite", glyph);
        parse_tilesprite::TileSpriteParser::parse_and_add(&filename, glyph, tr).unwrap();

        lh.glyph_names.push(glyph.to_string());
    }
}

fn init_colorsets(lh: &mut LayerHandler) {
    for colorset in COLORSETS {
        let tr = lh.get_renderer_mut();
        let filename = format!("assets/colorsets/{}.colorset", colorset);
        parse_colorset::ColorSetParser::parse_and_add(&filename, colorset, tr).unwrap();

        lh.color_names.push(colorset.to_string());
    }
}

fn construct_layers(lh: &mut LayerHandler) {
    let mut map = TileLayer::new("map".to_string(), 20, 18);
    map.set_z_index(0.);
    lh.add_layer(map);

    let mut ui = TileLayer::new("ui".to_string(), 20, 18);
    ui.set_z_index(10.);
    lh.add_layer(ui);

    let mut headingtext = TileLayer::new("headingtext".to_string(), 20, 1);
    headingtext.set_z_index(20.);
    lh.add_layer(headingtext);

    let mut fps = TileLayer::new("fps".to_string(), 20, 18);
    fps.set_z_index(100.);
    lh.add_layer(fps);
}

fn spawn_renderer(mut commands: Commands) {
    commands.spawn_bundle(TileRendererBundle::default());
}

#[cfg(debug_assertions)]
fn fps_counter(mut lh: ResMut<LayerHandler>, diagnostics: Res<Diagnostics>) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            let average = format!("{:03}", average.floor());
            lh.set_tile("fps", 17, 17, &average[0..=0], colors::Colors::Cursor)
                .unwrap();
            lh.set_tile("fps", 18, 17, &average[1..=1], colors::Colors::Cursor)
                .unwrap();
            lh.set_tile("fps", 19, 17, &average[2..=2], colors::Colors::Cursor)
                .unwrap();
        }
    }
}
