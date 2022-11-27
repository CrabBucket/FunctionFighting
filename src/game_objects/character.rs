use super::weapon::Weapon;
use bevy::prelude::*;
use std::sync::Arc;

#[derive(Component)]
struct CharacterName {
    name: String,
}
#[derive(Component)]
struct CurrentHealth {
    health: f32,
}
#[derive(Component)]
struct MaxHealth {
    health: f32,
}

#[derive(Bundle)]
struct CharStats {
    name: CharacterName,
    current_health: CurrentHealth,
    max_health: MaxHealth,
}

#[derive(Component)]
struct Inventory {
    items: Vec<Arc<dyn Weapon + Send + Sync>>,
}
#[derive(Component)]
struct EquippedWeapons {
    weapons: Vec<Arc<dyn Weapon + Send + Sync>>,
}

#[derive(Component, Clone, Copy)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    inventory: Inventory,
    equipped: EquippedWeapons,
    _player: Player,

    #[bundle]
    stats: CharStats,
    #[bundle]
    transform: TransformBundle,
    #[bundle]
    model: PbrBundle,
}
impl PlayerBundle {
    pub fn test(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        Self {
            inventory: Inventory { items: Vec::new() },
            equipped: EquippedWeapons {
                weapons: Vec::new(),
            },
            _player: Player,
            model: PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            stats: CharStats {
                name: CharacterName {
                    name: "Player".to_string(),
                },
                current_health: CurrentHealth { health: 100.0 },
                max_health: MaxHealth { health: 100.0 },
            },
            transform: TransformBundle {
                local: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
        }
    }
    pub fn get_player(&self) -> Player {
        self._player
    }
}
#[derive(Component)]
struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    stats: CharStats,
    weapons: EquippedWeapons,
    _enemy: Enemy,
}
