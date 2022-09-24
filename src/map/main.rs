use bevy::prelude::*;
use bevy::sprite::Rect;
use bevy_inspector_egui::Inspectable;

use crate::{EMPTY_VEC3, DEFAULT_SPRITE_SCALE};

#[derive(Component, Debug, Inspectable)]
pub struct Map;

impl Map {
  fn trading_arena_sprite_texture() -> Vec<Rect> {
    let x_pos = 224.;
    let y_pos = 351.;
    let initial_sprite_pos = Vec2::new(x_pos, y_pos);
    let rect = Rect {
        min: initial_sprite_pos,
        max: Vec2::new(
            x_pos + 240.,
            y_pos + 170.,
        ),
    };

    vec![rect]
}


    pub fn sprite_bundle(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) -> SpriteSheetBundle {
        let texture_handle = asset_server.load("sprites/trading_sprites_fill.png");
        let mut texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(180. , 150.), 1, 1);
        texture_atlas.textures = Map::trading_arena_sprite_texture();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let sprite_transform = Transform {
            translation: EMPTY_VEC3,
            scale: Vec3::new(4., 4., 0.),
            ..Default::default()
        };

        return SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: sprite_transform,
            ..Default::default()
        };
    }

    pub fn spawn_map(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let sprite_bundle = Map::sprite_bundle(asset_server, texture_atlases);
        commands
            .spawn()
            .insert(Name::new("Map"))
            .insert(Map)
            .insert_bundle(sprite_bundle);
    }
}
