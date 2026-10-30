use bevy::prelude::*;
pub struct Weapon {
    pub cooldown_time: f32,
    pub cooldown_timer: Timer,
    pub damage: i32
}