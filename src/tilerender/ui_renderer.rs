use crate::tilerender::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

pub struct UiRendererPlugin;

impl Plugin for UiRendererPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_ui_renderer.system());

        app.add_system(fps_counter.system());
    }
}

pub struct UiRenderer;

fn init_ui_renderer(mut commands: Commands, mut tr_assets: ResMut<Assets<TileRenderer>>) {
    let mut tilerenderer = TileRenderer::new(20, 18, 8, 8);
    tilerenderer.add_colorset_arr(
        "grayscale",
        &[
            (0, core::image::Rgba([9, 10, 22, 255])),
            (1, core::image::Rgba([99, 118, 126, 255])),
            (2, core::image::Rgba([160, 173, 178, 255])),
            (3, core::image::Rgba([224, 232, 237, 255])),
        ],
    );
    parse_tilesprite::TileSpriteParser::parse_and_add(
        "assets/fonts/lower.tilesprite",
        &mut tilerenderer,
    )
    .unwrap();
    parse_tilesprite::TileSpriteParser::parse_and_add(
        "assets/fonts/glyphs.tilesprite",
        &mut tilerenderer,
    )
    .unwrap();

    // let all_tileids = tilerenderer.get_all_tile_ids().clone();
    // for (idx, tileid) in all_tileids.iter().enumerate() {
    //     tilerenderer.set_tile(idx % 20, idx / 20, tileid, "grayscale");
    // }

    commands
        .spawn_bundle(TileRendererBundle {
            tilerenderer: tr_assets.add(tilerenderer),
            transform: Transform::from_xyz(0., 0., 100.),
            global_transform: GlobalTransform::default(),
        })
        .insert(UiRenderer);
}

fn fps_counter(
    tilerenderer: Query<&Handle<TileRenderer>, With<UiRenderer>>,
    mut tr_assets: ResMut<Assets<TileRenderer>>,
    diagnostics: Res<Diagnostics>,
) {
    if let Ok(tr) = tilerenderer.single() {
        let tr = tr_assets.get_mut(tr.id).unwrap();
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                let average = format!("{:03}", average);
                tr.set_tile(17, 17, &format!("lower_{}", &average[0..=0]), "grayscale");
                tr.set_tile(18, 17, &format!("lower_{}", &average[1..=1]), "grayscale");
                tr.set_tile(19, 17, &format!("lower_{}", &average[2..=2]), "grayscale");
            }
        }
    }
}
