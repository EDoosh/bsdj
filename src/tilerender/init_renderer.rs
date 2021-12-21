use crate::tilerender::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

pub struct InitRendererPlugin;

impl Plugin for InitRendererPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let mut tr = TileRenderer::new(8, 8);
        init_map_renderer(&mut tr);
        init_ui_renderer(&mut tr);

        let mut lh = LayerHandler::new(tr);
        construct_layers(&mut lh);
        app.insert_resource(lh);

        app.add_startup_system(spawn_renderer.system());
        app.add_system(fps_counter.system());
    }
}

fn init_map_renderer(tr: &mut TileRenderer) {
    tr.add_colorset_arr(
        "lushgreen",
        &[
            (0, core::image::Rgba([13, 41, 24, 255])),
            (1, core::image::Rgba([98, 161, 80, 255])),
            (2, core::image::Rgba([158, 227, 137, 255])),
            (3, core::image::Rgba([215, 255, 213, 255])),
        ],
    );
    parse_tilesprite::TileSpriteParser::parse_and_add("assets/map/map.tilesprite", tr).unwrap();

    add_map_clusters(tr);
}

fn add_map_clusters(tilerenderer: &mut TileRenderer) {
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

fn init_ui_renderer(tr: &mut TileRenderer) {
    tr.add_colorset_arr(
        "grayscale",
        &[
            (0, core::image::Rgba([9, 10, 22, 255])),
            (1, core::image::Rgba([99, 118, 126, 255])),
            (2, core::image::Rgba([160, 173, 178, 255])),
            (3, core::image::Rgba([224, 232, 237, 255])),
        ],
    );
    parse_tilesprite::TileSpriteParser::parse_and_add("assets/fonts/lower.tilesprite", tr).unwrap();
    parse_tilesprite::TileSpriteParser::parse_and_add("assets/fonts/glyphs.tilesprite", tr)
        .unwrap();

    // let all_tileids = tilerenderer.get_all_tile_ids().clone();
    // for (idx, tileid) in all_tileids.iter().enumerate() {
    //     tilerenderer.set_tile(idx % 20, idx / 20, tileid, "grayscale");
    // }
}

fn construct_layers(lh: &mut LayerHandler) {
    lh.add_layer(TileLayer::new("map".to_string(), 20, 18));
    lh.add_layer(TileLayer::new("ui".to_string(), 20, 18));
}

fn spawn_renderer(mut commands: Commands) {
    commands.spawn_bundle(TileRendererBundle::default());
}

fn fps_counter(mut lh: ResMut<LayerHandler>, diagnostics: Res<Diagnostics>) {
    if let Some(uirenderer) = lh.get_layer_mut("ui") {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                let average = format!("{:03}", average.floor());
                uirenderer.set_tile(17, 17, &format!("lower_{}", &average[0..=0]), "grayscale");
                uirenderer.set_tile(18, 17, &format!("lower_{}", &average[1..=1]), "grayscale");
                uirenderer.set_tile(19, 17, &format!("lower_{}", &average[2..=2]), "grayscale");
            }
        }
    }
}
