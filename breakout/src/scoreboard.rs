use crate::color::{SCORE_COLOR, TEXT_COLOR};
use bevy::prelude::*;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

pub struct Scoreboard {
    pub score: usize,
}

impl Scoreboard {
    pub fn text_bundle_sections(asset_server: Res<AssetServer>) -> TextBundle {
        let score_text = TextSection::new(
            "Score: ",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: SCOREBOARD_FONT_SIZE,
                color: TEXT_COLOR,
            },
        );

        let score_value = TextSection::from_style(TextStyle {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: SCOREBOARD_FONT_SIZE,
            color: SCORE_COLOR,
        });

        let section_position = Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
            ..default()
        };

        TextBundle::from_sections([score_text, score_value]).with_style(section_position)
    }
}
