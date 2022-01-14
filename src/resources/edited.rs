use bevy::prelude::*;

pub struct EditedPlugin;

impl Plugin for EditedPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EditedChain(0));
        app.insert_resource(EditedPhrase(0));
        app.insert_resource(EditedInstrument(0));
    }
}

pub struct EditedChain(pub u8);
pub struct EditedPhrase(pub u8);
pub struct EditedInstrument(pub u8);
