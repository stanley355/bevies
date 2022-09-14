use crate::color::BRICK_COLOR;
use crate::paddle::PADDLE_Y;
use crate::wall::{LEFT_WALL, RIGHT_WALL, TOP_WALL};
use bevy::prelude::*;

pub const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
// These values are exact
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
pub const GAP_BETWEEN_BRICKS: f32 = 5.0;
// These values are lower bounds, as the number of bricks is computed
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

const TOTAL_WIDTH_OF_BRICKS: f32 = (RIGHT_WALL - LEFT_WALL) - (2. * GAP_BETWEEN_BRICKS_AND_SIDES);
const BOTTOM_EDGE_OF_BRICKS: f32 = PADDLE_Y + GAP_BETWEEN_PADDLE_AND_BRICKS;
const TOTAL_HEIGHT_OF_BRICKS: f32 =
    TOP_WALL - BOTTOM_EDGE_OF_BRICKS - GAP_BETWEEN_BRICKS_AND_CEILING;

// Given the space available, compute how many rows and columns of bricks we can fit
pub const N_COLUMNS: usize = (TOTAL_WIDTH_OF_BRICKS / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)) as usize;
pub const N_ROWS: usize = (TOTAL_HEIGHT_OF_BRICKS / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)) as usize;
pub const N_VERTICAL_GAPS: usize = N_COLUMNS - 1;

// Because we need to round the number of columns,
// the space on the top and sides of the bricks only captures a lower bound, not an exact value
pub const CENTER_OF_BRICKS: f32 = (LEFT_WALL + RIGHT_WALL) / 2.0;
pub const LEFT_EDGE_OF_BRICKS: f32 = CENTER_OF_BRICKS
    - (N_COLUMNS as f32 / 2.0 * BRICK_SIZE.x) // Space taken up by the bricks
    - N_VERTICAL_GAPS as f32 / 2.0 * GAP_BETWEEN_BRICKS; // Space taken up by the gaps

// In Bevy, the `translation` of an entity describes the center point,
// not its bottom-left corner
pub const OFFSET_X: f32 = LEFT_EDGE_OF_BRICKS + BRICK_SIZE.x / 2.;
pub const OFFSET_Y: f32 = BOTTOM_EDGE_OF_BRICKS + BRICK_SIZE.y / 2.;

#[derive(Component)]
pub struct Brick;

impl Brick {
    pub fn brick_sprite_bundle(brick_position: Vec2) -> SpriteBundle {
        SpriteBundle {
            sprite: Sprite {
                color: BRICK_COLOR,
                ..default()
            },
            transform: Transform {
                translation: brick_position.extend(0.),
                scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
                ..default()
            },
            ..default()
        }
    }
}
