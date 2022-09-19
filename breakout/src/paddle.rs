use crate::breakout_plugin::TIME_STEP;
use crate::color::PADDLE_COLOR;
use crate::wall::{BOTTOM_WALL, LEFT_WALL, RIGHT_WALL, WALL_THICKNESS};
use bevy::prelude::*;

// Using the default 2D camera they correspond 1:1 with screen pixels.
const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;
const PADDLE_SPEED: f32 = 500.0;

// How close can the paddle get to the wall
const PADDLE_PADDING: f32 = 10.0;

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

    pub fn move_paddle(
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<&mut Transform, With<Paddle>>,
    ) {
        let mut paddle_transform = query.single_mut();
        let mut direction = 0.0;

        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        // Calculate the new horizontal paddle position based on player input
        let new_paddle_position =
            paddle_transform.translation.x + direction * PADDLE_SPEED * TIME_STEP;

        // Update the paddle position,
        // making sure it doesn't cause the paddle to leave the arena
        let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
        let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

        paddle_transform.translation.x = new_paddle_position.clamp(left_bound, right_bound);
    }
}
