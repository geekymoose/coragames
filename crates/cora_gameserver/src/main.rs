use bevy::prelude::*;

use crate::resources::turn_timer::TurnTimer;

mod components;
mod entities;
mod resources;

const TURN_DURATION_IN_SEC: u32 = 1;

fn main() {
    println!("--- Cora GameServer starts ---");

    let turn_timer = TurnTimer {
        timer: Timer::from_seconds(TURN_DURATION_IN_SEC as f32, TimerMode::Repeating),
        turn_duration_in_sec: TURN_DURATION_IN_SEC,
    };

    App::new().insert_resource(turn_timer).run();

    println!("--- Cora GameServer stops ---");
}
