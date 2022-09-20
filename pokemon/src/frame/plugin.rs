use super::title::Title;
use bevy::prelude::*;

#[derive(Debug)]
pub struct FramePlugin;

impl FramePlugin {
    pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let title_bundle = Title::text_bundle(asset_server);
        commands.spawn_bundle(title_bundle);
    }
}

impl Plugin for FramePlugin {
    fn build(&self, app: &mut App, ) {
        app.insert_resource(Title)
            .add_startup_system(FramePlugin::setup);
    }
}
