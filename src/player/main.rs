use bevy::prelude::*;
use bevy::sprite::Rect;
use bevy_inspector_egui::Inspectable;

use crate::{DEFAULT_SPRITE_SCALE, EMPTY_VEC2};

pub const INITIAL_PLAYER_X_POS: f32 = 25.;
pub const INITIAL_PLAYER_Y_POS: f32 = 4.;

pub const PLAYER_SPRITE_WIDTH: f32 = 19.;
pub const PLAYER_SPRITE_HEIGHT: f32 = 25.;

#[derive(Debug, Inspectable)]
pub enum FaceDirection {
    Up,
    Down,
    // Left,
    // Right,
}

#[derive(Component, Debug, Inspectable)]
pub struct Player;

impl Player {
    fn sprite_textures(face: FaceDirection) -> Vec<Rect> {
        let mut min_y = INITIAL_PLAYER_Y_POS;
        let max_y = min_y + PLAYER_SPRITE_HEIGHT;

        match face {
            FaceDirection::Down => min_y = INITIAL_PLAYER_Y_POS,
            FaceDirection::Up => min_y *= 3.0
        }

        let initial_sprite_pos = Vec2::new(INITIAL_PLAYER_X_POS, min_y);
        let rect = Rect {
            min: initial_sprite_pos,
            max: Vec2::new(INITIAL_PLAYER_X_POS + PLAYER_SPRITE_WIDTH, max_y),
        };

        vec![rect]
    }

    pub fn texture_atlas_handle(
        sprite_textures: Vec<Rect>,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) -> Handle<TextureAtlas> {
        let texture_handle = asset_server.load("sprites/player_sprites.png");
        let mut texture_atlas = TextureAtlas::from_grid(texture_handle, EMPTY_VEC2, 1, 1);
        texture_atlas.textures = sprite_textures;
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        return texture_atlas_handle;
    }

    pub fn sprite_bundle(
        asset_server: Res<AssetServer>,
        texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) -> SpriteSheetBundle {
        let sprite_textures = Player::sprite_textures(FaceDirection::Down);
        let texture_atlas_handle =
            Player::texture_atlas_handle(sprite_textures, asset_server, texture_atlases);

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

    pub fn move_player(
        keyboard: Res<Input<KeyCode>>,
        mut query: Query<&mut Transform, With<Player>>,
    ) {
        let mut player_transform = query.single_mut();

        let mut x_direction = 0.0;
        let mut y_direction = 0.0;

        if keyboard.pressed(KeyCode::Left) {
            x_direction -= 2.;
        }

        if keyboard.pressed(KeyCode::Right) {
            x_direction += 2.;
        }

        if keyboard.pressed(KeyCode::Up) {
            y_direction += 2.;
        }

        if keyboard.pressed(KeyCode::Down) {
            y_direction -= 2.;
        }

        let new_x_pos = player_transform.translation.x + x_direction;
        let new_y_pos = player_transform.translation.y + y_direction;

        player_transform.translation.x = new_x_pos;
        player_transform.translation.y = new_y_pos;
    }

    pub fn move_player_direction(
        keyboard: Res<Input<KeyCode>>,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut query: Query<&mut Handle<TextureAtlas>, With<Player>>,
    ) {
        let mut handle = query.single_mut();

        let texture_handle = asset_server.load("sprites/player_sprites.png");
        let mut texture_atlas_grid = TextureAtlas::from_grid(texture_handle, EMPTY_VEC2, 1, 1);
        texture_atlas_grid.textures = Player::sprite_textures(FaceDirection::Up);
        
        let texture_atlas_handle = &texture_atlases.add(texture_atlas_grid);

        let texture_atlas = texture_atlases.get(&texture_atlas_handle).unwrap();
        // let handle_2 = texture_atlas.add(texture_atlas_grid);
        
        // handle = texture_atlas_handle;
        // let mut x_direction = 0.0;
        // let mut y_direction = 0.0;

        // if keyboard.pressed(KeyCode::Left) {
        //     x_direction -= 2.;
        // }

        // if keyboard.pressed(KeyCode::Right) {
        //     x_direction += 2.;
        // }

        // if keyboard.pressed(KeyCode::Up) {
        //     y_direction += 2.;
        // }

        // if keyboard.pressed(KeyCode::Down) {
        //     y_direction -= 2.;
        // }

        // let new_x_pos = player_transform.translation.x + x_direction;
        // let new_y_pos = player_transform.translation.y + y_direction;

        // player_transform.translation.x = new_x_pos;
        // player_transform.translation.y = new_y_pos;
    }
}
