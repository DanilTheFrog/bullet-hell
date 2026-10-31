use bevy::prelude::*;
use super::components::*;
use super::input::*;

pub(super) fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let img: Handle<Image> = assets.load("idle.png");

    commands.spawn((
        Player,
        Player::default_transform(),
        Sprite {
            image: img,
            color: Color::linear_rgb(0.5, 0.9, 0.5),
            rect: Some(Rect{max: Vec2 { x: 30.0, y: 32.0 },..Default::default()}),
            ..Default::default()
        },
    ));
}

pub(super) fn move_player(mut q: Single<&mut Transform, With<Player>>, move_vec: Res<MovementVec>, t: Res<Time>) {
    const SPEED: f32 = 380.0;
    let res = move_vec.0 * SPEED * t.delta_secs();
    q.translation.x += res.x;
    q.translation.y += res.y;
}