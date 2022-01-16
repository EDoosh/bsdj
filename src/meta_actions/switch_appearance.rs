use crate::resources::input::{InputRes, InputType};
use crate::states::LoadState;
use crate::tilerender::*;
use bevy::prelude::*;

pub struct SwitchAppearancePlugin;

impl Plugin for SwitchAppearancePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(CoreStage::PreUpdate, switch_palette);
    }
}

fn switch_palette(
    mut lh: ResMut<LayerHandler>,
    inputs: Res<InputRes>,
    mut reload_state: ResMut<LoadState>,
) {
    let switch_back = inputs.just_pressed(&InputType::Key(KeyCode::F1));
    let switch_forth = inputs.just_pressed(&InputType::Key(KeyCode::F2));

    if switch_back || switch_forth {
        let colors = &lh.color_names;
        let current_index = colors
            .iter()
            .position(|c| c == &lh.active_colorset)
            .expect("Couldn't find current active colorset in list of all colorsets?");

        // Find the index of the new palette.
        let new_index = current_index as isize + if switch_forth { 1 } else { -1 };
        let new_index = new_index.rem_euclid(colors.len() as isize);

        lh.active_colorset = colors[new_index as usize].clone();
        reload_state.0 = true;
    }
}
