use bevy::{ecs::resource::Resource, time::Timer};


#[derive(Resource)]
pub(super) struct EnemySpawnTimer(pub(super) Timer);