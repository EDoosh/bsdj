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

fn enter_game(mut commands: Commands) {}

fn update_game(mut commands: Commands) {}

fn exit_game(mut commands: Commands) {}
