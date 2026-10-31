use bevy::prelude::*;
use super::weapons::Weapon;


#[derive(Resource)]
pub(super) struct Weapons {
    pub weapons: Vec<Weapon>
}

impl Default for Weapons {
    fn default() -> Self {
        let rifle = Weapon {
            // cooldown_time: 1f32,
            cooldown_timer: Timer::from_seconds(1f32, TimerMode::Repeating),
            // damage: 3,
        };
        Self {weapons: vec![rifle]}
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct BulletEntry {
    pub material: Handle<ColorMaterial>,
    pub mesh: Handle<Mesh>
}

#[derive(Resource, Default)]
pub struct BulletsRes {
    pub default: Option<BulletEntry>
}