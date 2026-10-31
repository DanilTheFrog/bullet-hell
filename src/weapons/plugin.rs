use bevy::prelude::*;

use crate::common::prelude::CombatSet;
use crate::weapons::systems::*;
use crate::weapons::*;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Weapons>();
        app.init_resource::<BulletsRes>();
        app.add_message::<WantsFire>();
        app.add_message::<ShouldFire>();

        app.add_systems(Startup, init_bullets_res);
        app.add_systems(
            FixedUpdate,
            (
                tick_weapons_cd,
                select_targets.in_set(CombatSet::TargetSelection),
                fire,
                // update_proj_pos,
            ).chain(),
        );
    }
}
