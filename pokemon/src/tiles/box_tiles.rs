use bevy::{prelude::*, sprite::Rect};

const BOX_TILES_WIDTH: f32 = 160.;
const BOX_TILES_HEIGHT: f32 = 115.;

#[derive(Debug)]
pub enum BoxTiles {
    Default,
}

impl BoxTiles {
    pub fn new(
        self,
        asset_server: &Res<AssetServer>,
        texture_atlas_res: &mut ResMut<Assets<TextureAtlas>>,
    ) -> SpriteSheetBundle {
        let texture_handle = BoxTiles::texture_atlas_handler(asset_server, texture_atlas_res);
        let transform = BoxTiles::set_transform();

        let sprite_sheet = SpriteSheetBundle {
            texture_atlas: texture_handle,
            transform: transform,
            ..default()
        };

        return sprite_sheet;
    }

    pub fn texture_atlas_handler(
        asset_server: &Res<AssetServer>,
        texture_atlas_res: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Handle<TextureAtlas> {
        let texture_asset = asset_server.load("sprites/box-sprites.png");
        let mut texture_atlas = TextureAtlas::from_grid(texture_asset, Vec2::new(0.0, 0.0), 1, 1);
        texture_atlas.textures = vec![Rect {
            min: Vec2::new(0.0, (600.0 + 145. * 4.)),
            max: Vec2::new(160.0, (600.0 + 145. * 4.) + 145.),
        }];

        let texture_atlas_handle = texture_atlas_res.add(texture_atlas);
        return texture_atlas_handle;
    }

    pub fn set_transform() -> Transform {
        Transform {
            translation: Vec3::new(0., 0., 0.),
            scale: Vec3::new(1., 1., 0.0),
            ..default()
        }
    }
}
