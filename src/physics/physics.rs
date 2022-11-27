use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::*;



pub fn move_player(query: Query<&ActionState<PlayerAction>, With<Player>>) {
    for action_state in query.iter() {
        if action_state.pressed(PlayerAction::MoveUp) {
            println!("Move up");
        }
        if action_state.pressed(PlayerAction::MoveDown) {
            println!("Move down");
        }
        if action_state.pressed(PlayerAction::MoveLeft) {
            println!("Move left");
        }
        if action_state.pressed(PlayerAction::MoveRight) {
            println!("Move right");
        }
    }
} 