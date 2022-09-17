use crate::ball::{Ball, BALL_SPEED, INITIAL_BALL_DIRECTION};
use crate::bricks::*;
use crate::collider::check_for_collisions;
use crate::collider::{Collider, CollisionEvent};
use crate::color::BACKGROUND_COLOR;
use crate::paddle::Paddle;
use crate::scoreboard::Scoreboard;
use crate::velocity::Velocity;
use crate::wall::{WallBundle, WallLocation};
use bevy::{prelude::*, window};

// Defines the amount of time that should elapse between each physics step.
// const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Debug)]
pub struct BreakoutPlugin;

impl Plugin for BreakoutPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scoreboard { score: 0 })
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_startup_system(setup)
            .add_event::<CollisionEvent>()
            .add_system_set(
                SystemSet::new()
                    // .with_run_criteria(FixedTimeStep::step(TIME_STEP as f64))
                    .with_system(check_for_collisions),
            )
            .add_system(window::close_on_esc);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Scoreboard
    let text_bundle_sections = Scoreboard::text_bundle_sections(asset_server);
    commands.spawn_bundle(text_bundle_sections);

    // TODO: Add ball collision sound after collison logic is done

    // Paddle
    // TODO: .insert(Collider) after inserting paddle bundle
    let paddle_sprite_bundle = Paddle::paddle_sprite_bundle();
    commands
        .spawn()
        .insert(Paddle)
        .insert_bundle(paddle_sprite_bundle);

    // Ball
    // TODO: .insert(Velocity(...)) after inserting velocity logic
    let ball_sprite_bundle = Ball::ball_sprite_bundle();
    commands
        .spawn()
        .insert(Ball)
        .insert_bundle(ball_sprite_bundle)
        .insert(Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED));

    // Walls
    commands.spawn_bundle(WallBundle::new(WallLocation::Left));
    commands.spawn_bundle(WallBundle::new(WallLocation::Right));
    commands.spawn_bundle(WallBundle::new(WallLocation::Bottom));
    commands.spawn_bundle(WallBundle::new(WallLocation::Top));

    // Bricks
    for row in 0..N_ROWS {
        for column in 0..N_COLUMNS {
            let offset_x = OFFSET_X + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS);
            let offset_y = OFFSET_Y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS);
            let brick_position = Vec2::new(offset_x, offset_y);

            let brick_sprite_bundle = Brick::brick_sprite_bundle(brick_position);
            commands
                .spawn()
                .insert(Brick)
                .insert_bundle(brick_sprite_bundle)
                .insert(Collider);
        }
    }
}
