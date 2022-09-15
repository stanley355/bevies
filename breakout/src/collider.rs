use bevy::prelude::*;
use crate::{ball::Ball, bricks::Brick, scoreboard::Scoreboard, velocity::Velocity};

#[derive(Component)]
pub struct Collider;

#[derive(Default)]
pub struct CollisionEvent;

// pub fn check_for_collisions(
//     mut commands: Commands,
//     mut scoreboard: ResMut<Scoreboard>,
//     mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
//     mut collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
    
// ) {
// }
