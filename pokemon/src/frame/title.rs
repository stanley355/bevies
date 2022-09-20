use bevy::prelude::*;

const GAME_TITLE: &str = "Rust Pokemon";
const TITLE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const TITLE_PADDING: Val = Val::Px(10.);

#[derive(Debug)]
pub struct Title;

impl Title {
    pub fn text_bundle(asset_server: Res<AssetServer>) -> TextBundle {
        let style = Title::text_style(asset_server);
        let position = Title::text_position();
        TextBundle::from_section(GAME_TITLE, style).with_style(position)
    }

    // TextStyle.font is required to make text visible
    pub fn text_style(asset_server: Res<AssetServer>) -> TextStyle {
        TextStyle {
            color: TITLE_COLOR,
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 24.
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
