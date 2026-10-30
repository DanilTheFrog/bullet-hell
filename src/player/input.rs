use std::process::exit;

use bevy::prelude::*;

#[derive(Resource)]
pub struct MovementVec(pub Vec2);


#[derive(Resource)]
pub struct ActionVec(pub Vec2);


pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MovementVec(Vec2::default()));
        app.insert_resource(ActionVec(Vec2::default()));
        app.add_systems(Update, update_move_desktop);
    }
}

fn update_move_desktop(mut move_vec: ResMut<MovementVec>, kb: Res<ButtonInput<KeyCode>>) {
    let mut dir = Vec2::ZERO;

    if kb.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if kb.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    if kb.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if kb.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }

    if kb.just_pressed(KeyCode::Escape) {
        exit(0);
    }

    dir = dir.normalize_or_zero();
    move_vec.0 = dir;
}