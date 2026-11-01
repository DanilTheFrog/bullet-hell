use bevy::prelude::*;
use super::systems::*;
use super::states_sets::GameState;
pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (update_pos).chain().run_if(in_state(GameState::Running)));
    }
}