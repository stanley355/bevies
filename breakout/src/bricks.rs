use crate::paddle::PADDLE_Y;
use crate::wall::{LEFT_WALL, RIGHT_WALL, TOP_WALL};
use bevy::prelude::*;

const BRICK_SIZE: Vec2 = Vec2::new(100., 30.);
// These values are exact
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
const GAP_BETWEEN_BRICKS: f32 = 5.0;
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



// // Because we need to round the number of columns,
// // the space on the top and sides of the bricks only captures a lower bound, not an exact value
// let center_of_bricks = (LEFT_WALL + RIGHT_WALL) / 2.0;
// let left_edge_of_bricks = center_of_bricks
//     // Space taken up by the bricks
//     - (n_columns as f32 / 2.0 * BRICK_SIZE.x)
//     // Space taken up by the gaps
//     - n_vertical_gaps as f32 / 2.0 * GAP_BETWEEN_BRICKS;
// // In Bevy, the `translation` of an entity describes the center point,
// // not its bottom-left corner
// let offset_x = left_edge_of_bricks + BRICK_SIZE.x / 2.;
// let offset_y = bottom_edge_of_bricks + BRICK_SIZE.y / 2.;


#[derive(Component)]
pub struct Brick;
