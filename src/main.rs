use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
mod window;

pub const WINDOW_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        .insert_resource(ClearColor(WINDOW_COLOR))
        .insert_resource(window::window_descriptor())
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}
