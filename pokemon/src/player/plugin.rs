use bevy::prelude::*;

use super::setup::PlayerSetup;

#[derive(Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(PlayerSetup::sprite_setup);
    }
}
