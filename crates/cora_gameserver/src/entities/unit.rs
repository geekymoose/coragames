use bevy::ecs::system::Commands;

use crate::components::health::Health;

fn spawn_unit(mut commands: Commands) {
    let health = Health {
        current_amount: todo!("WIP (to get from the config)"),
        max_amount: todo!("WIP (to get from the config)"),
    };
    commands.spawn(health);
}
