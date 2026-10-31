use bevy::prelude::*;
use bullet_hell::GamePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
            .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("A game"),
                        ..Default::default()
                    }),
                    ..default()
                })
            .set(ImagePlugin::default_nearest())
        )
        // .add_plugins(bevy_ecs_tilemap::TilemapPlugin)
        // .add_plugins(bullet_hell::helpers::tiled::TiledMapPlugin)
        .add_plugins(GamePlugin)
        .run();
}