use bevy::prelude::*;

use crate::weapons::Weapons;
use crate::weapons::systems::*;

pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Weapons>();
        app.add_systems(Update, (tick_weapons_cd, fire));
    }
}