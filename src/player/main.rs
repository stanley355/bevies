use std::ops::Add;

use bevy::prelude::*;
use bevy::sprite::Rect;
use bevy_inspector_egui::Inspectable;

use crate::{DEFAULT_SPRITE_SCALE, EMPTY_VEC2, EMPTY_VEC3};

pub const INITIAL_PLAYER_X_POS: f32 = 24.;
pub const INITIAL_PLAYER_Y_POS: f32 = 35.;

pub const PLAYER_SPRITE_WIDTH: f32 = 16.;
pub const PLAYER_SPRITE_HEIGHT: f32 = 21.;

#[derive(Component, Debug, Inspectable)]
pub struct Player;

impl Player {
    fn default_sprite_texture() -> Vec<Rect> {
        let initial_sprite_pos = Vec2::new(INITIAL_PLAYER_X_POS, INITIAL_PLAYER_Y_POS);
        let rect = Rect {
            min: initial_sprite_pos,
            max: Vec2::new(
                INITIAL_PLAYER_X_POS + PLAYER_SPRITE_WIDTH,
                INITIAL_PLAYER_Y_POS + PLAYER_SPRITE_HEIGHT,
            ),
        };

        vec![rect]
    }

    pub fn sprite_bundle(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) -> SpriteSheetBundle {
        let texture_handle = asset_server.load("sprites/player_sprites.png");
        let mut texture_atlas = TextureAtlas::from_grid(texture_handle, EMPTY_VEC2, 1, 1);
        texture_atlas.textures = Player::default_sprite_texture();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let sprite_transform = Transform {
            translation: EMPTY_VEC2.extend(1.),
            scale: DEFAULT_SPRITE_SCALE,
            ..Default::default()
        };

        return SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: sprite_transform,
            ..Default::default()
        };
    }

    pub fn spawn_player(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let sprite_bundle = Player::sprite_bundle(asset_server, texture_atlases);
        commands
            .spawn()
            .insert(Name::new("Player"))
            .insert(Player)
            .insert_bundle(sprite_bundle);
    }
}
