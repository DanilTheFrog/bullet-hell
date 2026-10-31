use bevy::prelude::*;
use crate::common::{Health, Team, Speed};
#[derive(Component)]
#[require(Health(10), Team = Team::Enemy, Speed(150f32))]
pub struct Enemy;

impl Enemy {
    pub const TRANSFORM_Z: f32 = 10.0; 
    pub fn default_transform() -> Transform {
        Transform {
            translation: Vec3 {z: 10.0, ..Default::default()},
            scale: Vec3 { x: 5.0, y: 5.0, z: 5.0 },

            ..Default::default()
        }
    }
}