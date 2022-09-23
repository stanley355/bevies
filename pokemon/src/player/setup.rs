use bevy::prelude::*;

use super::sprites::PlayerSprites;

#[derive(Debug)]
pub struct PlayerSetup;

impl PlayerSetup {
    pub fn sprite_setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_res: ResMut<Assets<TextureAtlas>>,
    ) {
        let player_sprites = PlayerSprites::new(&asset_server, &mut texture_atlas_res);
        commands.spawn_bundle(player_sprites);
    }
}
