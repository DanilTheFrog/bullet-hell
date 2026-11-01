use bevy::prelude::*;

#[derive(Component, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Health(pub i32);
impl From<i32> for Health {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

#[derive(Component, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Team {
    Player,
    Enemy
}

#[derive(Component, Default, PartialEq, Clone, Copy)]
pub struct Velocity(pub Vec2);

impl From<Vec2> for Velocity {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}

// #[derive(Component, PartialEq, Clone, Copy, Hash)]
// pub struct ChasesPlayer;

#[derive(Component, PartialEq, Clone, Copy)]
pub struct Speed(pub f32);

impl From<f32> for Speed {
    fn from(value: f32) -> Self {
        Self(value)
    }
}