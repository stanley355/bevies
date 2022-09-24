use bevy::prelude::*;
// use bevy::sprite::Rect;
use bevy_inspector_egui::Inspectable;

use crate::{EMPTY_VEC3, DEFAULT_SPRITE_SCALE};

#[derive(Component, Debug, Inspectable)]
pub struct Map;

impl Map {

    pub fn sprite_bundle(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) -> SpriteSheetBundle {
        let texture_handle = asset_server.load("design/home_design.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(180. , 150.), 1, 1);
        // texture_atlas.textures = Player::default_sprite_texture();
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let sprite_transform = Transform {
            translation: EMPTY_VEC3,
            scale: DEFAULT_SPRITE_SCALE,
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
