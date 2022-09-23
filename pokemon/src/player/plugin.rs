use super::{border::Border, title::Title};
use bevy::prelude::*;

#[derive(Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(PlayerPlugin::setup);
    }
}
