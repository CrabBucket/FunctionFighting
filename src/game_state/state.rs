use bevy::prelude::{Vec3, Resource};



#[derive(Resource, Default, Copy, Clone)]
pub struct Board {
    pub width: u32,
    pub height: u32,
}
#[derive(Resource, Default)]
pub struct GameState {
    board: Board,
    money: f32,
    time: u32,
    round: u8,
    camera_should_focus: Vec3,
    camera_is_focus: Vec3,
}
impl GameState {
    pub fn getBoard(&self) -> Board {
        self.board
    }
}
#[derive(Resource, Default, PartialEq, Eq, Debug, Clone, Copy, Hash)] 
pub enum GameStates {
    #[default]
    MainMenu,
    Playing,
    Shopping,
    Paused,
    GameOver,
}