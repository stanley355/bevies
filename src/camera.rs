use crate::{player::main::Player, RESOLUTION};
use bevy::prelude::*;

#[derive(Debug)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::camera_setup)
            .add_system(Self::camera_follow);
    }
}

impl CameraPlugin {
    fn camera_setup(mut commands: Commands) {
        let mut camera = Camera2dBundle::default();

        //Set the camera to have normalized coordinates of y values -1 to 1
        camera.projection.top = 1.0;
        camera.projection.bottom = -1.0;

        camera.projection.right = 1.0 * RESOLUTION;
        camera.projection.left = -1.0 * RESOLUTION;

        commands.spawn_bundle(camera);
    }

    fn camera_follow(
        player_query: Query<&Transform, With<Player>>,
        mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
    ) {
        let player_transform = player_query.single();
        let mut camera_transform = camera_query.single_mut();

        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}
