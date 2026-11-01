use bevy::{ecs::template::OptionTemplate::Some, prelude::*};

use super::components::Velocity;

pub(super) fn update_pos(
    mut q: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (mut trans, vel) in q.iter_mut() {
        trans.translation.x += vel.0.x * time.delta_secs();
        trans.translation.y += vel.0.y * time.delta_secs();
    }
}