use crate::frame::border::{BORDER_THICKNESS, BORDER_X_POSITION, BORDER_Y_POSITION};
use crate::tiles::box_tiles::{BoxTiles, BoxTilesType, BOX_TILES_HEIGHT, BOX_TILES_WIDTH};
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
        let second_floor_back_wall =
            PlayerHouse::second_floor_back_wall(&asset_server, &mut texture_atlas_res);

        commands.spawn_bundle(second_floor_back_wall);
        let second_floor_tiles =
            PlayerHouse::second_floor_tiles(&asset_server, &mut texture_atlas_res);
        commands.spawn_bundle(second_floor_tiles);
    }

    pub fn second_floor_tiles(
        asset_server: &Res<AssetServer>,
        texture_atlas_res: &mut ResMut<Assets<TextureAtlas>>,
    ) -> SpriteSheetBundle {
        let y_pos = -(BORDER_Y_POSITION / 3.);

        let tiles = BoxTiles {
            box_type: BoxTilesType::Plywood,
            transform: Transform {
                translation: Vec3::new(0., y_pos, 0.),
                scale: Vec3::new(5., 4., 0.0),
                ..default()
            },
        };
        tiles.new(&asset_server, texture_atlas_res)
    }

    pub fn second_floor_back_wall(
        asset_server: &Res<AssetServer>,
        texture_atlas_res: &mut ResMut<Assets<TextureAtlas>>,
    ) -> SpriteSheetBundle {
        let y_pos = (BORDER_Y_POSITION / 2.) + (BOX_TILES_HEIGHT / 2.);

        let tiles = BoxTiles {
            box_type: BoxTilesType::Plywood,
            transform: Transform {
                translation: Vec3::new(0., y_pos, 1.),
                scale: Vec3::new(5., 2., 0.0),
                ..default()
            },
        };
        tiles.new(&asset_server, texture_atlas_res)
    }
}
