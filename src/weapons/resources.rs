use bevy::{ecs::resource::Resource, time::{Timer, TimerMode}};

use crate::weapons::Weapon;


#[derive(Resource)]
pub struct Weapons {
    pub weapons: Vec<Weapon>
}

impl Default for Weapons {
    fn default() -> Self {
        let rifle = Weapon {
            cooldown_time: 1f32,
            cooldown_timer: Timer::from_seconds(1f32, TimerMode::Repeating),
            damage: 3,
        };
        Self {weapons: vec![rifle]}
    }
}