use std::sync::Arc;

use bevy::prelude::*;

#[derive(Component)]
struct ItemName {
    name: String,
}
#[derive(Component)]
struct ItemDescription {
    description: String,
}




#[derive(Bundle)]
struct ItemStats {
    name: ItemName,
    description: ItemDescription,
    #[bundle]
    sprite: SpriteBundle, 
}


#[derive(Component)]
pub struct OperandValue {
    value: f32,
}

#[derive(Bundle)]
pub struct OperandBundle {
    stats: ItemStats,
    value: OperandValue,
}

#[derive(Component)]
pub enum FunctionOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Component)]
pub struct Operator;

#[derive(Bundle)]
pub struct OperatorBundle {
    value: FunctionOperator,
    _operator: Operator,
    #[bundle]
    stats: ItemStats,
}

#[derive(Component)]
pub struct Function;

#[derive(Component)]
pub struct FunctionArray {
    operands: Vec<OperandBundle>,
    operators: Vec<OperatorBundle>,
}

#[derive(Bundle)]
pub struct FunctionBundle {
    _function: Function,
    function_array: FunctionArray,
}