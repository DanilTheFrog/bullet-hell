use std::f32::consts::PI;

use bevy::prelude::*;

use crate::common::prelude::*;
use crate::player::Player;
use super::{Enemy, resources::EnemySpawnTimer};

pub fn spawn_enemy(mut commands: Commands, assets: Res<AssetServer>) {
    let img: Handle<Image> = assets.load("idle.png");

    commands.spawn((
        Enemy,
        Velocity::default(),
        Sprite {
            image: img,
            color: Color::linear_rgb(0.9, 0.5, 0.5),
            rect: Some(Rect{max: Vec2 { x: 30.0, y: 32.0 },..Default::default()}),
            ..Default::default()
        },
        Transform {
            translation: Vec3::new(500.0, 0.0, 10.0),
            scale: Vec3 { x: 5.0, y: 5.0, z: 5.0 },
            ..Default::default()
        }
    ));
}

pub fn follow_player(
    mut q: Query<(&mut Velocity, &Speed, &Transform), (With<Enemy>, Without<Player>)>,
    p: Single<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let target = p.translation.truncate();

    for e in q.iter_mut() {
        let vel = e.0.into_inner();
        let speed = e.1;
        let pos = e.2.translation.truncate();
        let dir = (target - pos).normalize_or_zero();
        let res_vel = dir * speed.0;
        vel.0 = vel.0.lerp(res_vel, 0.8);
    }
}

// pub fn update_position(
//     mut q: Query<(&mut Transform, &Velocity), With<Enemy>>,
//     time: Res<Time>
// ) {
//     let delta = time.delta_secs();
//     for e in q.iter_mut() {
//         let (mut t, v) = e;
//         t.translation += (v.0 * delta).extend(0.0);
//     }
// }

pub fn tick(mut timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());
}
pub fn spawn_enemies(
    mut commands: Commands,
    timer: Res<EnemySpawnTimer>,
    q: Single<&Transform, With<Player>>,
    assets: Res<AssetServer>,
) {
    if !timer.0.is_finished() { return; }

    const INNER_R: f32 = 1000.0;
    const OUTER_R: f32 = INNER_R + 500.0;

    let center = q.translation.truncate();

    let d = rand::random_range(INNER_R..OUTER_R);
    let a = rand::random_range(0f32..PI*2.0);

    let x = d * a.cos();
    let y = d * a.sin(); 

    let mut pos = Vec2::new(x, y);
    pos = pos + center;

    let img: Handle<Image> = assets.load("idle.png");

    commands.spawn((
        Enemy,
        Velocity::default(),
        Speed(120f32),
        Sprite {
            image: img,
            color: Color::linear_rgb(0.9, 0.5, 0.5),
            rect: Some(Rect{max: Vec2 { x: 30.0, y: 32.0 },..Default::default()}),
            ..Default::default()
        },
        Transform {
            translation: pos.extend(Enemy::TRANSFORM_Z),
            ..Enemy::default_transform()
        }
    ));

}