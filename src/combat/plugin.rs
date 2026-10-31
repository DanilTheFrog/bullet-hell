use bevy::prelude::*;

use crate::common::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(FixedUpdate, 
            (
                CombatSet::TargetSelection,
                CombatSet::DamageCalculation,
                CombatSet::Cleanp
            ).in_set(GameSet::Combat)
        );
    }
}