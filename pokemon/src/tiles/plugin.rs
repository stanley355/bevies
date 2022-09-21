use bevy::prelude::*;

#[derive(Debug)]
pub struct TilesPlugin;

impl TilesPlugin {
    pub fn setup(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlas_res: ResMut<Assets<TextureAtlas>>,
    ) {
        let texture_asset = asset_server.load("tiles/tiles.png");
        let texture_atlas = TextureAtlas::from_grid(texture_asset, Vec2::new(200.0, 200.0), 4, 4);
        let texture_atlas_handle = texture_atlas_res.add(texture_atlas);

        let sprite_transform = Transform {
            translation: Vec3::new(0., 0., 0.),
            scale: Vec3::new(2., 2., 0.0),
            ..default()
        };

        let sprite_sheet = SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: sprite_transform,
            ..default()
        };

        commands.spawn_bundle(sprite_sheet);
    }
}

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(TilesPlugin::setup);
    }
}
