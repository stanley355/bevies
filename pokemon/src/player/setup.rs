use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_res: ResMut<Assets<TextureAtlas>>,
) {
    // // 2nd floor
    // let second_floor_back_wall =
    //     PlayerHouse::second_floor_back_wall(&asset_server, &mut texture_atlas_res);
    // commands.spawn_bundle(second_floor_back_wall);

    // let second_floor_tiles = PlayerHouse::second_floor_tiles(&asset_server, &mut texture_atlas_res);
    // commands.spawn_bundle(second_floor_tiles);
}
