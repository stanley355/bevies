use crate::color::PADDLE_COLOR;
use crate::wall::BOTTOM_WALL;
use bevy::prelude::*;

// Using the default 2D camera they correspond 1:1 with screen pixels.
const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;

// Paddle
pub const PADDLE_Y: f32 = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;

#[derive(Component, Debug)]
pub struct Paddle;

impl Paddle {
    pub fn paddle_sprite_bundle() -> SpriteBundle {
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, PADDLE_Y, 500.0),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        }
    }
}
