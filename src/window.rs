use bevy::prelude::WindowDescriptor;

const WINDOW_SIZE: f32 = 900.;

pub fn window_descriptor() -> WindowDescriptor {
    WindowDescriptor {
        width: WINDOW_SIZE * 2.,
        height: WINDOW_SIZE,
        title: "Pokemon Rust and Bevy".to_string(),
        resizable: false,
        ..Default::default()
    }
}
