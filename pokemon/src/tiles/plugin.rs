use bevy::prelude::*;

use crate::tiles::box_tiles::BoxTiles;

#[derive(Debug)]
pub struct TilesPlugin;

impl TilesPlugin {
    pub fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_res: ResMut<Assets<TextureAtlas>>,
    ) {
        let boxa = BoxTiles::new(BoxTiles::Default, &asset_server, &mut texture_atlas_res);

        commands.spawn_bundle(boxa);
    }
}

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(TilesPlugin::setup);
    }
}
