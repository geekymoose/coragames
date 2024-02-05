use bevy::{
    ecs::system::{Res, ResMut},
    time::Time,
};

use crate::resources::turn_timer::TurnTimer;

pub fn update_turn(global_time: Res<Time>, turn_timer: ResMut<TurnTimer>) {
    // let dt = global_time.delta();
    // TODO WIP
}
