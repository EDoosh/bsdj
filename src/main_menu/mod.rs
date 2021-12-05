use crate::states;
use crate::tilerender::*;
use bevy::prelude::*;
use bevy_retrograde::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(states::States::MainMenu).with_system(enter_menu.system()),
        );
        app.add_system_set(
            SystemSet::on_update(states::States::MainMenu).with_system(update_menu.system()),
        );
        app.add_system_set(
            SystemSet::on_exit(states::States::MainMenu).with_system(exit_menu.system()),
        );
    }
}

fn enter_menu(mut commands: Commands, mut tilerenderer: Query<&TileRenderer, With<MapRenderer>>) {}

fn update_menu(mut commands: Commands) {}

fn exit_menu(mut commands: Commands) {}
