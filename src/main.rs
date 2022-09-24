use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_debug_lines::*;

mod window;
mod player;

pub const WINDOW_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(WINDOW_COLOR))
        .insert_resource(window::window_descriptor())
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::with_depth_test(true))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(player::plugin::PlayerPlugin)
        .run();
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

// TODO: Check why OrthographicCameraBundle is not in bevy
// fn spawn_camera(mut commands: Commands) {
//     let mut camera = OrthoGraphicCameraBundle::new

//     //Set the camera to have normalized coordinates of y values -1 to 1
//     camera.orthographic_projection.top = 1.0;
//     camera.orthographic_projection.bottom = -1.0;

//     camera.orthographic_projection.right = 1.0 * RESOLUTION;
//     camera.orthographic_projection.left = -1.0 * RESOLUTION;

//     //Force the camera to use our settings
//     camera.orthographic_projection.scaling_mode = ScalingMode::None;

//     commands.spawn_bundle(camera);
// }
