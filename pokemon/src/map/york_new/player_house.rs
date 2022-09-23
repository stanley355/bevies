use crate::tiles::box_tiles::{BoxTiles, BoxTilesType};
use bevy::prelude::*;

#[derive(Debug)]
pub struct PlayerHouse;

impl PlayerHouse {
    pub fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_res: ResMut<Assets<TextureAtlas>>,
    ) {
        // 2nd floor
        let second_floor_background =
            PlayerHouse::second_floor_background(&asset_server, &mut texture_atlas_res);
        commands.spawn_bundle(second_floor_background);
    }

    pub fn second_floor_background(
        asset_server: &Res<AssetServer>,
        texture_atlas_res: &mut ResMut<Assets<TextureAtlas>>,
    ) -> SpriteSheetBundle {
        let tiles = BoxTiles {
            box_type: BoxTilesType::Plywood,
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: Vec3::new(1., 1., 0.0),
                ..default()
            },
        };
        tiles.new(&asset_server, texture_atlas_res)
    }
}
