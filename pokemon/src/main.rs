use bevy::{prelude::*, window};

mod frame;
mod tiles;
mod map;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(frame::plugin::FramePlugin)
        .add_plugin(map::york_new::plugin::YorkNewPlugin)
        .add_plugin(player::plugin::PlayerPlugin)
        .add_system(window::close_on_esc)
        .run();
}
