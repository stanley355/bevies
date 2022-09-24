use crate::player::main::Player;
use bevy::prelude::*;

#[derive(Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Player::spawn_player);
    }
}
