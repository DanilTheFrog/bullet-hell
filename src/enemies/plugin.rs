use bevy::prelude::*;

use super::systems::*;
use super::resources::EnemySpawnTimer;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(5.0, TimerMode::Repeating)));
        app.add_systems(Startup, spawn_enemy);
        app.add_systems(FixedUpdate, follow_player);
        app.add_systems(Update, (tick, spawn_enemies));
    }
}
