use bevy::{prelude::*, window};

mod frame;
mod tiles;
mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(frame::plugin::FramePlugin)
        // .add_plugin(tiles::plugin::TilesPlugin)
        .add_system(window::close_on_esc)
        .run();
}
