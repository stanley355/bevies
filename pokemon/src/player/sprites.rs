use bevy::{prelude::*, sprite::Rect};

use crate::tiles::box_tiles::DEFAULT_VEC2;

pub const INITIAL_PLAYER_X_POS: f32 = 24.;
pub const INITIAL_PLAYER_Y_POS: f32 = 35.;

pub const PLAYER_SPRITE_WIDTH: f32 = 16.;
pub const PLAYER_SPRITE_HEIGHT: f32 = 21.;

#[derive(Debug, Copy, Clone)]
pub struct PlayerSprites;

impl PlayerSprites {
    pub fn new(
        asset_server: &Res<AssetServer>,
        texture_atlas_res: &mut ResMut<Assets<TextureAtlas>>,
    ) -> SpriteSheetBundle {
        let texture_handle = PlayerSprites::texture_atlas_handler(asset_server, texture_atlas_res);
        let transform = Transform {
            translation: Vec3::new(0., 0., 2.),
            scale: Vec3::new(3., 3., 0.0),
            ..default()
        };

        let sprite = SpriteSheetBundle {
            texture_atlas: texture_handle,
            transform: transform,
            ..default()
        };

        return sprite;
    }

    pub fn texture_atlas_handler(
        asset_server: &Res<AssetServer>,
        texture_atlas_res: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Handle<TextureAtlas> {
        let texture_asset = asset_server.load("sprites/player_sprites.png");
        let mut texture_atlas = TextureAtlas::from_grid(texture_asset, DEFAULT_VEC2, 1, 1);
        texture_atlas.textures = PlayerSprites::get_texture();

        let texture_atlas_handle = texture_atlas_res.add(texture_atlas);
        return texture_atlas_handle;
    }

    pub fn get_texture() -> Vec<Rect> {
        let initial_sprite_pos = Vec2::new(INITIAL_PLAYER_X_POS, INITIAL_PLAYER_Y_POS);
        let rect = Rect {
            min: initial_sprite_pos,
            max: Vec2::new(INITIAL_PLAYER_X_POS + PLAYER_SPRITE_WIDTH, INITIAL_PLAYER_Y_POS + PLAYER_SPRITE_HEIGHT),
        };

        vec![rect]
    }
}
