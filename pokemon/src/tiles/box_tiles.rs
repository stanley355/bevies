use bevy::{prelude::*, sprite::Rect};

pub const BOX_TILES_WIDTH: f32 = 160.;
pub const BOX_TILES_HEIGHT: f32 = 115.;
pub const DEFAULT_VEC2: Vec2 = Vec2::new(0., 0.);
pub const DEFAULT_BOX_SIZE: Vec2 = Vec2::new(BOX_TILES_WIDTH, BOX_TILES_HEIGHT);
pub const PLYWOOD_BOX_POSITION: Vec2 = Vec2::new(484.0, 1122.);


#[derive(Debug, Copy, Clone)]
pub enum BoxTilesType {
    Plywood,
}

impl BoxTilesType {
    pub fn get_texture(self) -> Vec<Rect> {
        let rect = self.get_rect();

        vec![rect]
    }

    pub fn get_rect(self) -> Rect {
        let mut rect = Rect {
            min: Vec2::new(0., 0.),
            max: DEFAULT_BOX_SIZE,
        };

        match self {
            BoxTilesType::Plywood => {
                rect.min = PLYWOOD_BOX_POSITION;
                rect.max = Vec2::new(
                    PLYWOOD_BOX_POSITION.x + DEFAULT_BOX_SIZE.x,
                    PLYWOOD_BOX_POSITION.y + DEFAULT_BOX_SIZE.y,
                );
            }
        }

        rect
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BoxTiles {
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
