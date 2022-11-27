mod game_objects;
mod game_state;
mod input;
mod physics;

use bevy::prelude::*;
use game_objects::*;
use game_state::*;
use leafwing_input_manager::prelude::*;
use physics::*;

use crate::input::PlayerAction;

fn main() {
    App::new()
        .init_resource::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .add_state(GameStates::MainMenu)
        .add_startup_system(setup)
        .add_system_set(SystemSet::on_enter(GameStates::Playing).with_system(setup))
        // .add_system_set(SystemSet::on_exit(GameStates::Playing).with_system(teardown))
        // .add_system_set(SystemSet::on_enter(GameStates::GameOver).with_system(display_score))
        // .add_system_set(SystemSet::on_update(GameStates::GameOver).with_system(gameover_keyboard))
        // .add_system_set(SystemSet::on_exit(GameStates::GameOver).with_system(teardown))
        .add_system(bevy::window::close_on_esc)
        .run();
}
