use crate::states;
use crate::tilerender::*;
use bevy::prelude::*;
use bevy_retrograde::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(states::States::Game).with_system(enter_game.system()),
        );
        app.add_system_set(
            SystemSet::on_update(states::States::Game).with_system(update_game.system()),
        );
        app.add_system_set(
            SystemSet::on_exit(states::States::Game).with_system(exit_game.system()),
        );
    }
}

fn enter_game(
    map_renderer: Query<&Handle<TileRenderer>, With<MapRenderer>>,
    mut tr_assets: ResMut<Assets<TileRenderer>>,
) {
    // if let Ok(mut ui_tf) = ui_renderer.single_mut() {
    //     let translation = &mut ui_tf.translation;
    //     translation.y += 128.;
    // }

    if let Ok(map_renderer) = map_renderer.single() {
        let map_renderer = tr_assets.get_mut(map_renderer.id).unwrap();
        map_renderer.clear_map("map_clear", "lushgreen");

        for i in 0..10 {
            for j in 0..9 {
                map_renderer.set_cluster(i * 2, j * 2, "map_grass")
            }
        }

        map_renderer.set_cluster(-2, -2, "map_tree");
        map_renderer.set_cluster(14, 4, "map_tree");
        map_renderer.set_cluster(2, 6, "map_flower");
        map_renderer.set_cluster(8, 6, "map_house");

        map_renderer.set_cluster(4, 2, "map_fence");
        map_renderer.set_cluster(6, 2, "map_fence");
        map_renderer.set_cluster(8, 2, "map_fence");
        map_renderer.set_cluster(10, 2, "map_fence");
        map_renderer.set_cluster(12, 2, "map_fence");
        map_renderer.set_cluster(14, 2, "map_fence");
        map_renderer.set_cluster(16, 2, "map_fence");
        map_renderer.set_cluster(4, 4, "map_fence");
        map_renderer.set_cluster(4, 6, "map_fence");
        map_renderer.set_cluster(4, 8, "map_fence");
        map_renderer.set_cluster(4, 10, "map_fence");
        map_renderer.set_cluster(4, 12, "map_fence");
        map_renderer.set_cluster(6, 12, "map_fence");
        map_renderer.set_cluster(8, 12, "map_fence");
        map_renderer.set_cluster(12, 12, "map_fence");
        map_renderer.set_cluster(14, 12, "map_fence");
        map_renderer.set_cluster(16, 12, "map_fence");
        map_renderer.set_cluster(16, 10, "map_fence");
        map_renderer.set_cluster(16, 8, "map_fence");

        map_renderer.set_cluster(6, 4, "map_longgrass");
        map_renderer.set_cluster(8, 4, "map_longgrass");
        map_renderer.set_cluster(10, 4, "map_longgrass");
        map_renderer.set_cluster(12, 4, "map_longgrass");
        map_renderer.set_cluster(18, 4, "map_longgrass");
        map_renderer.set_cluster(18, 6, "map_longgrass");
        map_renderer.set_cluster(6, 6, "map_longgrass");
        map_renderer.set_cluster(6, 8, "map_longgrass");
        map_renderer.set_cluster(6, 10, "map_longgrass");
        map_renderer.set_cluster(8, 10, "map_longgrass");
        map_renderer.set_cluster(12, 10, "map_longgrass");
        map_renderer.set_cluster(14, 10, "map_longgrass");
        map_renderer.set_cluster(14, 8, "map_longgrass");

        map_renderer.set_cluster(10, 10, "map_path");
        map_renderer.set_cluster(10, 12, "map_path");
        map_renderer.set_cluster(10, 14, "map_path");
        map_renderer.set_cluster(8, 14, "map_path");
        map_renderer.set_cluster(6, 14, "map_path");
        map_renderer.set_cluster(4, 14, "map_path");
        map_renderer.set_cluster(2, 14, "map_path");
        map_renderer.set_cluster(0, 14, "map_path");
    }
}

fn update_game(mut commands: Commands) {}

fn exit_game(mut commands: Commands) {}
