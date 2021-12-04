use crate::states;
use crate::tilerender::*;
use bevy::prelude::*;
use bevy_retrograde::prelude::*;

pub struct UiRendererPlugin;

impl Plugin for UiRendererPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_ui_renderer.system());
    }
}

pub struct UiRenderer;

fn init_ui_renderer(mut commands: Commands) {
    let mut tilerenderer = TileRenderer::new(160, 144, 8, 8);
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

    let all_tileids = tilerenderer.get_all_tile_ids().clone();
    for (idx, tileid) in all_tileids.iter().enumerate() {
        tilerenderer.set_tile(idx % 20, idx / 20, tileid, "grayscale");
    }

    commands
        .spawn_bundle(TileRendererBundle {
            tilerenderer,
            transform: Transform::from_xyz(-80., -72., 100.),
            global_transform: GlobalTransform::default(),
        })
        .insert(UiRenderer);
}
