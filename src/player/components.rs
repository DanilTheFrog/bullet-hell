use bevy::prelude::*;
use crate::common::{Health, Team};
#[derive(Component)]
#[require(Health(20), Team = Team::Player)]
pub struct Player;

impl Player {
    pub const TRANSFORM_Z: f32 = 15.0; 
    pub fn default_transform() -> Transform {
        Transform {
            translation: Vec3 {z: 15.0, ..Default::default()},
            scale: Vec3 { x: 5.0, y: 5.0, z: 5.0 },
            ..Default::default()
        }
    }
}