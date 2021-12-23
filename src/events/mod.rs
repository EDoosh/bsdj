use bevy::prelude::*;

pub mod heading_text;
pub use heading_text::HeadingTextEvent;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(heading_text::HeadingTextPlugin);
    }
}
