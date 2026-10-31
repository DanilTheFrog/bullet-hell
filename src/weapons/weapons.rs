use bevy::prelude::*;
pub(super) struct Weapon {
    // pub cooldown_time: f32,
    pub cooldown_timer: Timer,
    // pub damage: i32
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum WeaponTarget {
    Position(Vec2),
}