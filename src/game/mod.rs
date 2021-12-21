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

fn enter_game(mut lh: ResMut<LayerHandler>) {
    let tr = &lh.get_renderer().clone();
    if let Some(map_renderer) = lh.get_layer_mut("map") {
        map_renderer.clear_layer("map_clear", "lushgreen");

        for i in 0..10 {
            for j in 0..9 {
                map_renderer.set_cluster(tr, i * 2, j * 2, "map_grass")
            }
        }

        map_renderer.set_cluster(tr, -2, -2, "map_tree");
        map_renderer.set_cluster(tr, 14, 4, "map_tree");
        map_renderer.set_cluster(tr, 2, 6, "map_flower");
        map_renderer.set_cluster(tr, 8, 6, "map_house");

        map_renderer.set_cluster(tr, 4, 2, "map_fence");
        map_renderer.set_cluster(tr, 6, 2, "map_fence");
        map_renderer.set_cluster(tr, 8, 2, "map_fence");
        map_renderer.set_cluster(tr, 10, 2, "map_fence");
        map_renderer.set_cluster(tr, 12, 2, "map_fence");
        map_renderer.set_cluster(tr, 14, 2, "map_fence");
        map_renderer.set_cluster(tr, 16, 2, "map_fence");
        map_renderer.set_cluster(tr, 4, 4, "map_fence");
        map_renderer.set_cluster(tr, 4, 6, "map_fence");
        map_renderer.set_cluster(tr, 4, 8, "map_fence");
        map_renderer.set_cluster(tr, 4, 10, "map_fence");
        map_renderer.set_cluster(tr, 4, 12, "map_fence");
        map_renderer.set_cluster(tr, 6, 12, "map_fence");
        map_renderer.set_cluster(tr, 8, 12, "map_fence");
        map_renderer.set_cluster(tr, 12, 12, "map_fence");
        map_renderer.set_cluster(tr, 14, 12, "map_fence");
        map_renderer.set_cluster(tr, 16, 12, "map_fence");
        map_renderer.set_cluster(tr, 16, 10, "map_fence");
        map_renderer.set_cluster(tr, 16, 8, "map_fence");

        map_renderer.set_cluster(tr, 6, 4, "map_longgrass");
        map_renderer.set_cluster(tr, 8, 4, "map_longgrass");
        map_renderer.set_cluster(tr, 10, 4, "map_longgrass");
        map_renderer.set_cluster(tr, 12, 4, "map_longgrass");
        map_renderer.set_cluster(tr, 18, 4, "map_longgrass");
        map_renderer.set_cluster(tr, 18, 6, "map_longgrass");
        map_renderer.set_cluster(tr, 6, 6, "map_longgrass");
        map_renderer.set_cluster(tr, 6, 8, "map_longgrass");
        map_renderer.set_cluster(tr, 6, 10, "map_longgrass");
        map_renderer.set_cluster(tr, 8, 10, "map_longgrass");
        map_renderer.set_cluster(tr, 12, 10, "map_longgrass");
        map_renderer.set_cluster(tr, 14, 10, "map_longgrass");
        map_renderer.set_cluster(tr, 14, 8, "map_longgrass");

        map_renderer.set_cluster(tr, 10, 10, "map_path");
        map_renderer.set_cluster(tr, 10, 12, "map_path");
        map_renderer.set_cluster(tr, 10, 14, "map_path");
        map_renderer.set_cluster(tr, 8, 14, "map_path");
        map_renderer.set_cluster(tr, 6, 14, "map_path");
        map_renderer.set_cluster(tr, 4, 14, "map_path");
        map_renderer.set_cluster(tr, 2, 14, "map_path");
        map_renderer.set_cluster(tr, 0, 14, "map_path");
    }
}

fn update_game(mut commands: Commands) {}

fn exit_game(mut commands: Commands) {}
