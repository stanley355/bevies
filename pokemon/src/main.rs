use bevy::{prelude::*, window};

mod frame;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(frame::plugin::FramePlugin)
        .add_system(window::close_on_esc)
        .run();
}
