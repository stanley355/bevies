use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod map;
mod player;
mod window;
mod camera;

pub const EMPTY_VEC2: Vec2 = Vec2::new(0., 0.);
pub const EMPTY_VEC3: Vec3 = Vec3::new(0., 0., 0.);
pub const DEFAULT_SPRITE_SCALE: Vec3 = Vec3::new(3., 3., 0.);
pub const WINDOW_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(WINDOW_COLOR))
        .insert_resource(window::window_descriptor())
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(camera::CameraPlugin)
        .add_plugin(map::plugin::MapPlugin)
        .add_plugin(player::plugin::PlayerPlugin)
        .run();
}

