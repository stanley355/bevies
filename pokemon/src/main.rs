use bevy::{prelude::*, window};

mod frame;
mod tiles;
mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(frame::plugin::FramePlugin)
        .add_plugin(map::york_new::plugin::YorkNewPlugin)
        .add_system(window::close_on_esc)
        .run();
}
