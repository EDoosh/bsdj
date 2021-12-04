use bevy::prelude::*;
use bevy_retrograde::prelude::*;

mod game;
mod states;
mod tilerender;

const SCALE: f32 = 5.;

#[derive(StageLabel, Debug, Eq, Hash, PartialEq, Clone)]
struct GameStage;

/// Create the app and open the program.
fn main() {
    let mut app = App::build();

    // Set the properties of the window itself
    app.insert_resource(WindowDescriptor {
        title: "Topdown Project".to_string(),
        width: 160. * SCALE,
        height: 144. * SCALE,
        ..Default::default()
    });

    // region:      ADD THE PLUGINS
    app.add_plugins(RetroPlugins);
    app.add_plugin(game::GamePlugin);
    app.add_plugin(tilerender::TileRenderPlugin);
    // endregion:   ADD THE PLUGINS

    // Add the setup for the app.
    app.add_startup_system(setup.system());

    // Set the current game state to be the main game.
    app.add_state(states::States::Game);

    // RUN THE PROGRAM
    app.run();
}

fn setup(mut commands: Commands) {
    // Add a camera to the scene.
    commands.spawn().insert_bundle(CameraBundle {
        camera: Camera {
            // Set our camera to have a fixed height and width
            size: CameraSize::LetterBoxed {
                width: 160,
                height: 144,
            },
            background_color: Color::new(0.2, 0.2, 0.2, 1.),
            letterbox_color: Color::new(0., 0., 0., 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
