use bevy::{prelude::*, sprite::Rect};

const BOX_TILES_WIDTH: f32 = 160.;
const BOX_TILES_HEIGHT: f32 = 115.;

const DEFAULT_BOX_SIZE: Vec2 = Vec2::new(BOX_TILES_WIDTH, BOX_TILES_HEIGHT);
const PLYWOOD_BOX_POSITION: Vec2 = Vec2::new(484.0, 1122.);

#[derive(Debug)]
pub enum BoxTiles {
    Plywood,
}

impl BoxTiles {
    pub fn new(
        self,
        asset_server: &Res<AssetServer>,
        texture_atlas_res: &mut ResMut<Assets<TextureAtlas>>,
    ) -> SpriteSheetBundle {
        let texture_handle = self.texture_atlas_handler(asset_server, texture_atlas_res);
        let transform = BoxTiles::set_transform();

        let sprite_sheet = SpriteSheetBundle {
            texture_atlas: texture_handle,
            transform: transform,
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
        let mut texture_atlas = TextureAtlas::from_grid(texture_asset, Vec2::new(1000., 0.), 1, 1);
        texture_atlas.textures = self.set_box_rect();

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

    pub fn set_box_rect(self) -> Vec<Rect> {
        let mut rect = Rect {
            min: Vec2::new(0., 0.),
            max: DEFAULT_BOX_SIZE,
        };

        match self {
            BoxTiles::Plywood => {
                rect.min = PLYWOOD_BOX_POSITION;
                rect.max = Vec2::new(
                    PLYWOOD_BOX_POSITION.x + DEFAULT_BOX_SIZE.x,
                    PLYWOOD_BOX_POSITION.y + DEFAULT_BOX_SIZE.y,
                );
            }
        }

        vec![rect]
    }
}
