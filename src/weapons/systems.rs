use bevy::{color::palettes::css::YELLOW, prelude::*};

use crate::{player::Player, weapons::{Bullet, Weapons}};

pub(super) fn tick_weapons_cd(
    mut weapons: ResMut<Weapons>,
    time: Res<Time> 
) {
    let delta = time.delta();
    for w in weapons.weapons.iter_mut() {
        w.cooldown_timer.tick(delta);
    }
}

pub(super) fn fire(
    mut commands: Commands,
    weapons: Res<Weapons>,
    p: Single<&Transform, With<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = Mesh2d(meshes.add(Circle::new(10.0)));
    let material = MeshMaterial2d(materials.add(ColorMaterial::from_color(YELLOW)));
    let pos = p.translation;

    for w in &weapons.weapons {
        if !w.cooldown_timer.is_finished() { continue; }
        commands.spawn((
            mesh.clone(),
            material.clone(),
            Bullet {damage: w.damage},
            Transform::from_xyz(pos.x, pos.y, pos.z)
        ));
    }
}

