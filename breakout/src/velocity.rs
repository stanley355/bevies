use bevy::prelude::*;
#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

// TODO: Not working here!
// impl Velocity {
//     pub fn apply_velocity(mut query: Query<&mut Transform, &Velocity>) {
//         for (mut transform, velocity) in &mut query {
//             // transform.translation.x += velocity.x * TIME_STEP;
//             // transform.translation.y += velocity.y * TIME_STEP;
//         }
//     }
// }
