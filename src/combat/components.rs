use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[require(DamageType)]
pub struct Damage;

#[derive(Default, Component)]
pub enum DamageType {
    #[default]
    Physical,
    Magical
}

#[derive(Component)]
pub struct DamageValue(pub i32);

impl Default for DamageValue {
    fn default() -> Self {
        Self(3)
    }
}


impl From<i32> for DamageValue {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
