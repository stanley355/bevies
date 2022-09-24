use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Debug, Inspectable)]
pub struct Player;

impl Player {
    pub fn spawn_player(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let texture_handle = asset_server.load("sprites/player_sprites_fill.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(100.0, 100.0), 1, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let sprite_transform = Transform {
            translation: Vec3::new(10.0, 10.0, 10.),
            scale: Vec3::new(2., 2., 0.0),
            ..Default::default()
        };

        let sprite_bundle = SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: sprite_transform,
            ..Default::default()
        };

        commands
            .spawn()
            .insert(Name::new("Player"))
            .insert(Player)
            .insert_bundle(sprite_bundle);
    }
}
