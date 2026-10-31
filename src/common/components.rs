use bevy::{ecs::component::Component, math::Vec2};
use derive_more::{From, Into};

#[derive(Component, PartialEq, Eq, Clone, Copy, From, Into, Hash)]
pub struct Health(pub i32);

#[derive(Component, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Team {
    Player,
    Enemy
}

#[derive(Component, Default, PartialEq, Clone, Copy, From, Into)]
pub struct Velocity(pub Vec2);

// #[derive(Component, PartialEq, Clone, Copy, Hash)]
// pub struct ChasesPlayer;

#[derive(Component, PartialEq, Clone, Copy)]
pub struct Speed(pub f32);
