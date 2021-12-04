use crate::states;
use crate::tilerender::*;
use bevy::prelude::*;
use bevy_retrograde::prelude::*;

pub struct MapRendererPlugin;

impl Plugin for MapRendererPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(init_map_renderer.system());
    }
}

pub struct MapRenderer;

fn init_map_renderer(mut commands: Commands) {
    let mut tilerenderer = TileRenderer::new(160, 144, 8, 8);
    tilerenderer.add_colorset(
        "ocean",
        ColorSet::from_tuple(&[(0, core::image::Rgba([115, 202, 214, 255]))]),
    );
    tilerenderer.add_tilesprite_arr(" ", &[0; 8 * 8]).unwrap();
    tilerenderer.clear_map(" ", "ocean");

    commands
        .spawn_bundle(TileRendererBundle {
            tilerenderer,
            transform: Transform::from_xyz(-80., -72., 0.),
            global_transform: GlobalTransform::default(),
        })
        .insert(MapRenderer);
}
