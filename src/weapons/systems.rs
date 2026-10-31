use bevy::{color::palettes::css::YELLOW, prelude::*};

use crate::combat::{Damage, DamageType, DamageValue};
use crate::common::Velocity;
use crate::enemies::Enemy;
use crate::player::Player;
use crate::weapons::{ BulletEntry, BulletsRes, Projectile, ShouldFire, WantsFire, Weapons};
use super::weapons::WeaponTarget;

pub(super) fn tick_weapons_cd(
    mut weapons: ResMut<Weapons>,
    time: Res<Time>,
    mut writer: MessageWriter<WantsFire>
) {
    let delta = time.delta();
    for w in weapons.weapons.iter_mut() {
        w.cooldown_timer.tick(delta);
        if w.cooldown_timer.is_finished() { writer.write(WantsFire::default()); }
    }
}

pub(super) fn select_targets(
    targets: Query<&Transform, With<Enemy>>,
    player: Single<&Transform, With<Player>>,
    mut wants: MessageReader<WantsFire>,
    mut should: MessageWriter<ShouldFire>
) {
    let player_pos = player.translation.truncate();
    let nearest = targets
        .iter()
        .map(|x| x.translation.truncate())
        .min_by(|a, b| {
            let dist_a = (*a - player_pos).length_squared();
            let dist_b = (*b - player_pos).length_squared();
            
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
        });

    for _ in wants.read() {
        if let Some(target) = nearest {
            should.write(ShouldFire {damage: 5, target: WeaponTarget::Position(target)});
        }
    }
}

pub(super) fn fire(
    mut commands: Commands,
    mut reader: MessageReader<ShouldFire>,
    player: Single<&Transform, With<Player>>,
    bullets: Res<BulletsRes>
) {
    let def = bullets.default.as_ref().unwrap();

    let pos = player.translation.truncate();

    for w in reader.read() {
        let mesh = Mesh2d(def.mesh.clone());
        let material = MeshMaterial2d(def.material.clone());
        let dir = match w.target {
            WeaponTarget::Position(p) => (p - pos).normalize_or_zero()
        };

        let vel = dir * 400.0;

        commands.spawn((
            mesh.clone(),
            material.clone(),
            Projectile,
            Damage,
            Velocity(vel),
            DamageValue(w.damage),
            DamageType::Physical,
            Transform::from_xyz(pos.x, pos.y, Player::TRANSFORM_Z)
        ));
    }
}

pub(super) fn init_bullets_res(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut bullets: ResMut<BulletsRes>
) {
    let mesh = meshes.add(Circle::new(10.0));
    let material = materials.add(ColorMaterial::from_color(YELLOW));

    bullets.default = Some(BulletEntry { material, mesh});
}