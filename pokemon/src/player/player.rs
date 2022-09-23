use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Player;

impl Player {
    pub fn move_player(
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<&mut Transform, With<Player>>,
    ) {
        for mut player_transform in query.iter_mut() {
            let mut direction = 0.0;

            println!("{:?}", player_transform);
            if keyboard_input.pressed(KeyCode::Left) {
                println!("hihihihi");
                player_transform.translation.x = 100.;
            }

            if keyboard_input.pressed(KeyCode::Right) {
              player_transform.translation.x = -100.;
            }

        }
    }
}
