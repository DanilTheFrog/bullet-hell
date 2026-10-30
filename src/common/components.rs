use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub enum Team {
    Player,
    Enemy
}