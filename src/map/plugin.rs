use super::main::Map;
use bevy::prelude::*;

#[derive(Debug)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Map::spawn_map);
    }
}
