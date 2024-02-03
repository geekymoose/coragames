use bevy::{ecs::system::Resource, time::Timer};

#[derive(Resource)]
pub struct TurnTimer {
    pub timer: Timer,
    pub turn_duration_in_sec: f32,
    pub turns_counter: u32,
}
