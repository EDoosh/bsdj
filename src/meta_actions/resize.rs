use crate::resources::input::{InputRes, InputType};
use bevy::prelude::*;

pub struct ResizePlugin;

impl Plugin for ResizePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(resize_window);
    }
}

fn resize_window(mut windows: ResMut<Windows>, inputs: Res<InputRes>) {
    if inputs.is_pressed(&[InputType::Key(KeyCode::LControl)]) {
        let window = windows.get_primary_mut().unwrap();
        let mut scale = window.scale_factor();

        // Zoom in
        if inputs.just_pressed(&InputType::Key(KeyCode::Equals)) {
            scale += 1.;
        }

        // Zoom out
        if inputs.just_pressed(&InputType::Key(KeyCode::Minus)) {
            scale -= 1.;
        }

        // Above a scale of 4 breaks??
        window.set_scale_factor_override(Some(scale.clamp(1., 4.)));
    }
}
