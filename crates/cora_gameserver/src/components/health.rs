use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Health {
    pub current_amount: u32,
    pub max_amount: u32,
}
