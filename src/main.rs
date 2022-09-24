use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod map;
mod player;
mod window;

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
        .add_startup_system(camera_setup)
        .add_plugin(map::plugin::MapPlugin)
        .add_plugin(player::plugin::PlayerPlugin)
        .run();
}

fn camera_setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    //Set the camera to have normalized coordinates of y values -1 to 1
    camera.projection.top = 1.0;
    camera.projection.bottom = -1.0;

    camera.projection.right = 1.0 * RESOLUTION;
    camera.projection.left = -1.0 * RESOLUTION;

    commands.spawn_bundle(camera);
}
