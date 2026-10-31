use bevy::ecs::{component::Component, message::Message};
use super::weapons::WeaponTarget;


#[derive(Component)]
pub struct Projectile;

#[derive(Message, Debug, Default, PartialEq, Clone, Copy, Hash)]
pub struct WantsFire;

#[derive(Message, Debug, Clone, Copy, PartialEq)]
pub(super) struct ShouldFire {
    pub target: WeaponTarget,
    pub damage: i32
}