use bevy::prelude::*;

use crate::combat::CombatPlugin;
use crate::common::{CommonPlugin, prelude::*};
use crate::enemies::EnemiesPlugin;
use crate::level::LevelPlugin;
use crate::player::PlayerPlugin;
use crate::weapons::WeaponsPlugin;

mod common;
mod level;
mod player;
mod enemies;
mod weapons;
mod combat;
pub mod helpers;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.init_state::<GameStage>();
        app.configure_sets(Update, (GameSet::UiInput, GameSet::Input.run_if(in_state(GameState::Running))).chain());
        app.configure_sets(FixedUpdate, (GameSet::Movement, GameSet::Combat, GameSet::Cleanup).run_if(in_state(GameState::Running)).chain());
        app.add_plugins(CommonPlugin);
        app.add_plugins(LevelPlugin);
        app.add_plugins(PlayerPlugin);
        app.add_plugins(EnemiesPlugin);
        app.add_plugins(WeaponsPlugin);
        app.add_plugins(CombatPlugin);
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, scale_camera);
    }
}

#[derive(Component)]
pub struct MainCamera;
fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera2d,
        Transform {
            scale: Vec3 { x: 1.5, y: 1.5, z: 1.5 },
            ..Default::default()
        }
    ));
}

fn scale_camera(mut cam: Single<&mut Transform, With<MainCamera>>, kb: Res<ButtonInput<KeyCode>>) {

    if kb.just_pressed(KeyCode::ArrowDown) {
        cam.scale.x += 0.1;
        cam.scale.y += 0.1;
        cam.scale.z += 0.1;

    }
    if kb.just_pressed(KeyCode::ArrowUp) {
        cam.scale.x -= 0.1;
        cam.scale.y -= 0.1;
        cam.scale.z -= 0.1;
    }
}