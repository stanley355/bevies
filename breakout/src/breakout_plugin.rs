
use crate::ball::Ball;
use crate::color::BACKGROUND_COLOR;
use crate::paddle::Paddle;
use crate::scoreboard::Scoreboard;
use crate::wall::{WallBundle, WallLocation};
use bevy::{prelude::*, window};

#[derive(Debug)]
pub struct BreakoutPlugin;

impl Plugin for BreakoutPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scoreboard { score: 0 })
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_startup_system(setup)
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
        .insert_bundle(ball_sprite_bundle);

    // Walls
    commands.spawn_bundle(WallBundle::new(WallLocation::Left));
    commands.spawn_bundle(WallBundle::new(WallLocation::Right));
    commands.spawn_bundle(WallBundle::new(WallLocation::Bottom));
    commands.spawn_bundle(WallBundle::new(WallLocation::Top));
}
