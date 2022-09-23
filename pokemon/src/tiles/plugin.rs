use bevy::prelude::*;
use crate::tiles::box_tiles::BoxTiles;
use super::box_tiles::BoxTilesType;

pub const DEFAULT_VEC2: Vec2 = Vec2::new(0., 0.);

#[derive(Debug)]
pub struct TilesPlugin;

impl TilesPlugin {
    pub fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_res: ResMut<Assets<TextureAtlas>>,
    ) {
        let new_box = BoxTiles {
            box_type: BoxTilesType::Plywood,
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: Vec3::new(1., 1., 0.0),
                ..default()
            },
        };
        let boxa = new_box.new(&asset_server, &mut texture_atlas_res);

        commands.spawn_bundle(boxa);
    }
}

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(TilesPlugin::setup);
    }
}
