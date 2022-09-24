use bevy::prelude::*;
use crate::player::main::Player;

#[derive(Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Player::spawn_player.label("Helo"));
    }
}

