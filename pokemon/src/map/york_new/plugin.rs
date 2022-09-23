use bevy::prelude::*;

use super::player_house::PlayerHouse;

#[derive(Debug)]
pub struct YorkNewPlugin;

impl Plugin for YorkNewPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(PlayerHouse::setup);
    }
}
