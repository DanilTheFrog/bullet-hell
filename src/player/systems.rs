use bevy::prelude::*;
use crate::common::prelude::*;
use super::components::*;
use super::input::*;

pub(super) fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let img: Handle<Image> = assets.load("idle.png");

    commands.spawn((
        Player,
        Team::Player,
        Health(100),
        Sprite {
            image: img,
            color: Color::linear_rgb(0.5, 0.9, 0.5),
            rect: Some(Rect{max: Vec2 { x: 30.0, y: 32.0 },..Default::default()}),
            ..Default::default()
        },
        Transform {
            translation: Vec3 {z: 15.0, ..Default::default()},
            scale: Vec3 { x: 5.0, y: 5.0, z: 5.0 },
            ..Default::default()
        }
    ));
}

pub(super) fn move_player(mut q: Single<&mut Transform, With<Player>>, move_vec: Res<MovementVec>, t: Res<Time>) {
    const SPEED: f32 = 380.0;
    let res = move_vec.0 * SPEED * t.delta_secs();
    q.translation.x += res.x;
    q.translation.y += res.y;
}