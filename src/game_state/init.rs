use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;


use crate::*;

use super::GameState;

#[derive(Component)]
pub struct PhysicsPlugin;


fn create_camera(commands: &mut Commands){
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

}

fn setup_physics(commands: &mut Commands, world: GameState, player_entity: Entity) {
    let world_size = world.getBoard();

    /* Create the ground. */
    commands
        .spawn(PhysicsPlugin)
        .insert(Collider::cuboid(world_size.width as f32, 0.1, world_size.height as f32))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the player ball. */
    commands
        .get_or_spawn(player_entity)
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}

fn add_player(commands:&mut Commands, materials: &mut Assets<StandardMaterial>, meshes: &mut Assets<Mesh>) -> Entity {
    let player = PlayerBundle::test(meshes, materials);
    let input_map = [(KeyCode::W, PlayerAction::MoveUp), 
    (KeyCode::S, PlayerAction::MoveDown), 
    (KeyCode::A, PlayerAction::MoveLeft), 
    (KeyCode::D, PlayerAction::MoveRight)];
    let entity = commands.spawn(player)
    .insert(InputManagerBundle::<PlayerAction>{
        action_state: ActionState::default(),
        input_map: InputMap::new(input_map)
    })
    .id();

    return entity;
}

fn setup_scene(commands: &mut Commands, meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

pub fn setup(mut commands: Commands,gamestate: Res<GameState> ,materials: &mut ResMut<Assets<StandardMaterial>>, meshes: &mut ResMut<Assets<Mesh>>) {
    create_camera(&mut commands);
    let player = add_player(&mut commands, materials, meshes);
    setup_physics(&mut commands, *gamestate, player);
    setup_scene(&mut commands, meshes, materials);
}