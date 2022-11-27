use leafwing_input_manager::Actionlike;

#[derive(Actionlike, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerAction {
    Stay,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Shoot,
}