use bevy::{prelude::*, window};

mod frame;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera_setup)
        .add_plugin(frame::plugin::FramePlugin)
        .add_system(window::close_on_esc)
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
