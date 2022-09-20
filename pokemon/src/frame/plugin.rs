use bevy::prelude::*;
use super::title::Title;

#[derive(Debug)]
pub struct FramePlugin;

impl FramePlugin {
    pub fn setup(mut commands: Commands) {
        commands.spawn_bundle(Title::text_bundle());
    }
}

impl Plugin for FramePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Title)
            .add_startup_system(FramePlugin::setup);
    }
}
