use bevy::prelude::*;

const BORDER_X_AXIS: f32 = 600.;
const BORDER_THICKNESS: f32 = 10.;

pub enum Border {
    Left,
    Right,
    Bottom,
    Top,
}

impl Border {
    pub fn new(&self) -> SpriteBundle {
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
            Border::Left => Vec2::new(-BORDER_X_AXIS, 0.),
            _ => Vec2::new(0., 0.),
        }
    }

    pub fn size(&self) -> Vec2 {
        match self {
            Border::Left => Vec2::new(BORDER_THICKNESS, 100.),
            _ => Vec2::new(0., 0.),
        }
    }
}
