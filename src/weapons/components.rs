use bevy::ecs::component::Component;


#[derive(Component)]
pub struct Bullet {
    pub damage: i32,
}