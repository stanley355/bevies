use bevy::prelude::*;

pub const BORDER_X_POSITION: f32 = 410.;
pub const BORDER_Y_POSITION: f32 = 300.0;
pub const BORDER_THICKNESS: f32 = 10.;

pub enum Border {
    Left,
    Right,
    // Bottom,
    // Top,
}

impl Border {
    pub fn new(self) -> SpriteBundle {
        SpriteBundle {
            transform: Transform {
                translation: self.position().extend(0.0),
                scale: self.size().extend(0.0),
                ..default()
            },
            ..default()
        }
    }

    pub fn position(&self) -> Vec2 {
        match self {
            Border::Left => Vec2::new(-BORDER_X_POSITION, 0.),
            Border::Right => Vec2::new(BORDER_X_POSITION, 0.),
            // Border::Top => Vec2::new(0., BORDER_Y_POSITION),
            // Border::Bottom => Vec2::new(0., -BORDER_Y_POSITION),
        }
    }

    pub fn size(&self) -> Vec2 {
        // let frame_width = 2. * (BORDER_THICKNESS + BORDER_X_POSITION);
        let frame_height = 2. * (BORDER_THICKNESS + BORDER_Y_POSITION);

        match self {
            Border::Left | Border::Right => Vec2::new(BORDER_THICKNESS, frame_height),
            // Border::Top | Border::Bottom => Vec2::new(frame_width, BORDER_THICKNESS),
        }
    }
}
