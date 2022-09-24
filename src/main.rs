use bevy::prelude::*;
use bevy::render::camera::OrthoGraphicCameraBundle;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_debug_lines::*;

mod player;
mod window;

pub const EMPTY_VEC2: Vec2 = Vec2::new(0., 0.);
pub const EMPTY_VEC3: Vec3 = Vec3::new(0., 0., 0.);
pub const DEFAULT_SPRITE_SCALE: Vec3 = Vec3::new(3., 3., 0.);
pub const WINDOW_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(WINDOW_COLOR))
        .insert_resource(window::window_descriptor())
        .add_startup_system(camera_setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::with_depth_test(true))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(player::plugin::PlayerPlugin)
        .run();
}

fn camera_setup(mut commands: Commands) {
    let mut camera = OrthoGraphicCameraBundle::new_2d();
    commands.spawn_bundle(Camera2dBundle::default());
}

// TODO: Check why line plugin doesn't work
// #[derive(Debug)]
// pub struct LinePlugin;

// impl Plugin for LinePlugin {
//     fn build(&self, app: &mut App) {
//         app.add_system(line_system.label("Line System"));
//     }
// }

// fn line_system(
//     //  ...
//     mut lines: ResMut<DebugLines>,
// ) {
//     let start = Vec3::splat(-1.0);
//     let end = Vec3::splat(10.0);
//     let duration = 100.0; // Duration of 0 will show the line for 1 frame.
//     lines.line(start, end, duration);
// }

// TODO: Check why OrthographicCameraBundle is not in bevy 0.8
// fn spawn_camera(mut commands: Commands) {
//     let mut camera = OrthoGraphicCameraBundle::new_2d();

//     //Set the camera to have normalized coordinates of y values -1 to 1
//     camera.orthographic_projection.top = 1.0;
//     camera.orthographic_projection.bottom = -1.0;

//     camera.orthographic_projection.right = 1.0 * RESOLUTION;
//     camera.orthographic_projection.left = -1.0 * RESOLUTION;

//     //Force the camera to use our settings
//     camera.orthographic_projection.scaling_mode = ScalingMode::None;

//     commands.spawn_bundle(camera);
// }
