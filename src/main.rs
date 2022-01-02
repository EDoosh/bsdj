use bevy::{prelude::*, window::WindowMode};
// use bevy_inspector_egui::WorldInspectorPlugin;

mod events;
mod meta_actions;
mod resources;
mod scenes;
mod states;
mod tilerender;
mod utils;

const SCALE: f64 = 4.;

#[derive(StageLabel, Debug, Eq, Hash, PartialEq, Clone)]
struct GameStage;

/// Create the app and open the program.
fn main() {
    let mut app = App::new();

    // Set the properties of the window itself
    app.insert_resource(WindowDescriptor {
        title: "LSDj".to_string(),
        width: 160.,
        height: 144.,
        vsync: true,
        resizable: false,
        mode: WindowMode::Windowed,
        scale_factor_override: Some(SCALE),
        ..Default::default()
    });

    app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default());

    // region:      ADD THE PLUGINS
    app.add_plugins(DefaultPlugins);
    // app.add_plugin(WorldInspectorPlugin::new());
    app.add_plugin(events::EventsPlugin);
    app.add_plugin(resources::ResourcePlugin);
    app.add_plugin(scenes::ScenePlugin);
    app.add_plugin(tilerender::TileRenderPlugin);
    app.add_plugin(meta_actions::MetaActionsPlugin);
    // endregion:   ADD THE PLUGINS

    // Add the setup for the app.
    app.add_startup_system(setup);

    // Set the current game state to be the song screen.
    app.add_state(states::States::Song);

    // RUN THE PROGRAM
    app.run();
}

fn setup(mut commands: Commands) {
    // Add a camera to the scene.
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
