use bevy::prelude::{App, DefaultPlugins};

mod ball;
mod breakout_plugin;
mod bricks;
mod collider;
mod color;
mod paddle;
mod scoreboard;
mod wall;
mod velocity;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(breakout_plugin::BreakoutPlugin)
        .run();
}
