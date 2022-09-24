use bevy::prelude::*;

pub const WINDOW_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const WINDOW_SIZE: f32 = 900.;
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(WINDOW_COLOR))
        .insert_resource(WindowDescriptor {
            width: WINDOW_SIZE * 2.,
            height: WINDOW_SIZE,
            title: "Pokemon Rust and Bevy".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
