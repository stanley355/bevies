use bevy::prelude::*;

use super::player::Player;
use super::setup::PlayerSetup;

#[derive(Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(PlayerSetup::sprite_setup)
            .add_system_set(SystemSet::new().with_system(Player::move_player));
    }
}
