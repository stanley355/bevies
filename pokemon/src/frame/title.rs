use bevy::prelude::*;

const GAME_TITLE: &str = "Rust Pokemon";
const TITLE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const TITLE_PADDING: Val = Val::Px(5.);

#[derive(Debug)]
pub struct Title;

impl Title {
    pub fn text_bundle() -> TextBundle {
        let style = Title::text_style();
        let position = Title::text_position();
        TextBundle::from_section(GAME_TITLE, style).with_style(position)
    }

    pub fn text_style() -> TextStyle {
        TextStyle {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            color: TITLE_COLOR,
            ..default()
        }
    }

    pub fn text_position() -> Style {
        Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: TITLE_PADDING,
                left: TITLE_PADDING,
                ..default()
            },
            ..default()
        }
    }
}
