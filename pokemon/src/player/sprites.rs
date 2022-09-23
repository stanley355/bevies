use bevy::{prelude::*, sprite::Rect};

#[derive(Debug, Copy, Clone)]
pub struct PlayerSprites {
    pub box_type: BoxTilesType,
    pub transform: Transform,
}

impl BoxTiles {
    pub fn new(
        self,
        asset_server: &Res<AssetServer>,
        texture_atlas_res: &mut ResMut<Assets<TextureAtlas>>,
    ) -> SpriteSheetBundle {
        let texture_handle = self.texture_atlas_handler(asset_server, texture_atlas_res);

        let sprite_sheet = SpriteSheetBundle {
            texture_atlas: texture_handle,
            transform: self.transform,
            ..default()
        };

        return sprite_sheet;
    }

    pub fn texture_atlas_handler(
        self,
        asset_server: &Res<AssetServer>,
        texture_atlas_res: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Handle<TextureAtlas> {
        let texture_asset = asset_server.load("sprites/box-sprites.png");
        let mut texture_atlas = TextureAtlas::from_grid(texture_asset, DEFAULT_VEC2, 1, 1);
        texture_atlas.textures = self.box_type.get_texture();

        let texture_atlas_handle = texture_atlas_res.add(texture_atlas);
        return texture_atlas_handle;
    }
}
