use bevy::prelude::*;

pub struct EditedPlugin;

impl Plugin for EditedPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EditedChain(0));
        app.insert_resource(EditedPhrase(0));
        app.insert_resource(EditedInstrument(0));
        app.insert_resource(EditedTable(0));
        app.insert_resource(EditedGroove(0));
        app.insert_resource(EditedSynth(0));
        app.insert_resource(EditedWaveframe(0));
        app.insert_resource(EditedWord(0));
    }
}

pub struct EditedChain(pub u8);
pub struct EditedPhrase(pub u8);
pub struct EditedInstrument(pub u8);
pub struct EditedTable(pub u8);
pub struct EditedGroove(pub u8);
pub struct EditedSynth(pub u8);
pub struct EditedWaveframe(pub u8);
pub struct EditedWord(pub u8);
