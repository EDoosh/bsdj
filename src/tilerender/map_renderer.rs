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

fn init_map_renderer(mut commands: Commands, mut tr_assets: ResMut<Assets<TileRenderer>>) {
    let mut tilerenderer = TileRenderer::new(20, 18, 8, 8);
    tilerenderer.add_colorset_arr(
        "lushgreen",
        &[
            (0, core::image::Rgba([13, 41, 24, 255])),
            (1, core::image::Rgba([98, 161, 80, 255])),
            (2, core::image::Rgba([158, 227, 137, 255])),
            (3, core::image::Rgba([215, 255, 213, 255])),
        ],
    );
    parse_tilesprite::TileSpriteParser::parse_and_add(
        "assets/map/map.tilesprite",
        &mut tilerenderer,
    )
    .unwrap();

    add_clusters(&mut tilerenderer);

    commands
        .spawn_bundle(TileRendererBundle {
            tilerenderer: tr_assets.add(tilerenderer),
            transform: Transform::from_xyz(0., 0., 0.),
            global_transform: GlobalTransform::default(),
        })
        .insert(MapRenderer);
}

fn add_clusters(tilerenderer: &mut TileRenderer) {
    tilerenderer.add_new_cluster(
        "map_tree",
        4,
        4,
        &[
            Tile::new("map_tree_11", "lushgreen"),
            Tile::new("map_tree_21", "lushgreen"),
            Tile::new("map_tree_31", "lushgreen"),
            Tile::new("map_tree_41", "lushgreen"),
            Tile::new("map_tree_12", "lushgreen"),
            Tile::new("map_tree_22", "lushgreen"),
            Tile::new("map_tree_32", "lushgreen"),
            Tile::new("map_tree_42", "lushgreen"),
            Tile::new("map_tree_13", "lushgreen"),
            Tile::new("map_tree_23", "lushgreen"),
            Tile::new("map_tree_33", "lushgreen"),
            Tile::new("map_tree_43", "lushgreen"),
            Tile::new("map_tree_14", "lushgreen"),
            Tile::new("map_tree_24", "lushgreen"),
            Tile::new("map_tree_34", "lushgreen"),
            Tile::new("map_tree_44", "lushgreen"),
        ],
    );
    tilerenderer.add_new_cluster(
        "map_grass",
        2,
        2,
        &[
            Tile::new("map_short_grass", "lushgreen"),
            Tile::new("map_clear", "lushgreen"),
            Tile::new("map_clear", "lushgreen"),
            Tile::new("map_short_grass", "lushgreen"),
        ],
    );
    tilerenderer.add_new_cluster(
        "map_longgrass",
        2,
        2,
        &[
            Tile::new("map_long_grass", "lushgreen"),
            Tile::new("map_long_grass", "lushgreen"),
            Tile::new("map_long_grass", "lushgreen"),
            Tile::new("map_long_grass", "lushgreen"),
        ],
    );
    tilerenderer.add_new_cluster(
        "map_path",
        2,
        2,
        &[
            Tile::new("map_path", "lushgreen"),
            Tile::new("map_path", "lushgreen"),
            Tile::new("map_path", "lushgreen"),
            Tile::new("map_path", "lushgreen"),
        ],
    );
    tilerenderer.add_new_cluster(
        "map_fence",
        2,
        2,
        &[
            Tile::new("map_fence_post_top", "lushgreen"),
            Tile::new("map_fence_post_top", "lushgreen"),
            Tile::new("map_fence_post", "lushgreen"),
            Tile::new("map_fence_post", "lushgreen"),
        ],
    );
    tilerenderer.add_new_cluster(
        "map_flower",
        2,
        2,
        &[
            Tile::new("map_flower", "lushgreen"),
            Tile::new("map_short_grass", "lushgreen"),
            Tile::new("map_short_grass", "lushgreen"),
            Tile::new("map_flower", "lushgreen"),
        ],
    );
    tilerenderer.add_new_cluster(
        "map_house",
        6,
        4,
        &[
            Tile::new("map_house_roof_top", "lushgreen"),
            Tile::new("map_house_roof_top", "lushgreen"),
            Tile::new("map_house_roof_top", "lushgreen"),
            Tile::new("map_house_roof_top", "lushgreen"),
            Tile::new("map_house_roof_top", "lushgreen"),
            Tile::new("map_house_roof_top", "lushgreen"),
            Tile::new("map_house_roof_bottom", "lushgreen"),
            Tile::new("map_house_roof_bottom", "lushgreen"),
            Tile::new("map_house_roof_bottom", "lushgreen"),
            Tile::new("map_house_roof_bottom", "lushgreen"),
            Tile::new("map_house_roof_bottom", "lushgreen"),
            Tile::new("map_house_roof_bottom", "lushgreen"),
            Tile::new("map_house_window_11", "lushgreen"),
            Tile::new("map_house_window_21", "lushgreen"),
            Tile::new("map_house_entrance_11", "lushgreen"),
            Tile::new("map_house_entrance_21", "lushgreen"),
            Tile::new("map_house_window_11", "lushgreen"),
            Tile::new("map_house_window_21", "lushgreen"),
            Tile::new("map_house_window_12", "lushgreen"),
            Tile::new("map_house_window_22", "lushgreen"),
            Tile::new("map_house_entrance_12", "lushgreen"),
            Tile::new("map_house_entrance_22", "lushgreen"),
            Tile::new("map_house_window_12", "lushgreen"),
            Tile::new("map_house_window_22", "lushgreen"),
        ],
    );
}
