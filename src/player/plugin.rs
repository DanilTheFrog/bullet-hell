use bevy::prelude::*;

use crate::{common::prelude::*, player::InputPlugin};
use super::systems::*;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputPlugin);
        app.add_systems(OnEnter(GameState::Running), spawn_player);
        app.add_systems(FixedUpdate, move_player
            .run_if(in_state(GameState::Running)).in_set(GameSet::Movement));
    }
}

