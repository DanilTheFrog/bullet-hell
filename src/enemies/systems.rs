use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{common::Health, enemies::{Enemy, EnemySpawnTimer}, player::Player};


pub fn spawn_enemy(mut commands: Commands, assets: Res<AssetServer>) {
    let img: Handle<Image> = assets.load("idle.png");

    commands.spawn((
        Enemy,
        Health(10),
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
    mut q: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    p: Single<&Transform, (With<Player>, Without<Enemy>)>,
    t: Res<Time>
) {
    const SPEED: f32 = 100.0;
    let target = p.translation.truncate();

    for mut e in q.iter_mut() {
        let pos = e.translation.truncate();
        let mut next = target - pos;
        next = next.normalize_or_zero();
        next = next * SPEED * t.delta_secs();
        e.translation += next.extend(0.0);
    }
}

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
    info!("{:?}", pos);

    let img: Handle<Image> = assets.load("idle.png");

    commands.spawn((
        Enemy,
        Health(10),
        Sprite {
            image: img,
            color: Color::linear_rgb(0.9, 0.5, 0.5),
            rect: Some(Rect{max: Vec2 { x: 30.0, y: 32.0 },..Default::default()}),
            ..Default::default()
        },
        Transform {
            translation: pos.extend(10.0),
            scale: Vec3 { x: 5.0, y: 5.0, z: 5.0 },
            ..Default::default()
        }
    ));

}