use super::*;
use bevy::prelude::*;

#[derive(Component)]
struct WeaponName(String);

#[derive(Component)]
struct WeaponDescription(String);



#[derive(Bundle)]
pub struct WeaponStats {
    name: WeaponName,
    description: WeaponDescription,
    
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Component)]
pub struct RangedWeapon;

#[derive(Bundle)]
pub struct RangedWeaponBundle {
    _ranged_weapon: RangedWeapon,
    
    #[bundle]
    barrel_functions: FunctionBundle,
    #[bundle]
    damage_functions: FunctionBundle,
    #[bundle]
    fire_rate_functions: FunctionBundle,
    #[bundle]
    magazine_functions: FunctionBundle,

    #[bundle]
    stats: WeaponStats,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Component)]
pub struct MeleeWeapon;
#[derive(Bundle)]
pub struct MeleeWeaponBundle {
    _melee_weapon: MeleeWeapon,
    
    #[bundle]
    damage_functions: FunctionBundle,
    #[bundle]
    fire_rate_functions: FunctionBundle,
    #[bundle]
    magazine_functions: FunctionBundle,

    #[bundle]
    stats: WeaponStats,
    #[bundle]
    sprite: SpriteBundle,
}   



pub(crate) trait Weapon {
    fn get_name(&self) -> & String;
    fn get_description(&self) -> & String;
    fn get_sprite(&self) -> & SpriteBundle;
}

impl Weapon for RangedWeaponBundle {
    fn get_name(&self) -> & String {
        &self.stats.name.0
    }
    fn get_description(&self) -> & String {
        &self.stats.description.0
    }
    fn get_sprite(&self) -> & SpriteBundle {
        &self.sprite
    }
}
impl Weapon for MeleeWeaponBundle {
    fn get_name(&self) -> & String {
        &self.stats.name.0
    }
    fn get_description(&self) -> & String {
        &self.stats.description.0
    }
    fn get_sprite(&self) -> & SpriteBundle {
        &self.sprite
    }
}