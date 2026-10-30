use bevy::prelude::*;

use crate::{player::{InputPlugin}};
use super::systems::*;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputPlugin);
        app.add_systems(Startup, spawn_player);
        app.add_systems(FixedUpdate, move_player);
    }
}

