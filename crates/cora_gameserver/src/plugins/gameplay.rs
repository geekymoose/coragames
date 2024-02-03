use bevy::{
    app::{App, Plugin, Update},
    time::{Timer, TimerMode},
};

use crate::resources::turn_timer::TurnTimer;

const TURN_DURATION_IN_SEC: f32 = 1.0;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        let turn_timer = TurnTimer {
            timer: Timer::from_seconds(TURN_DURATION_IN_SEC, TimerMode::Repeating),
            turn_duration_in_sec: TURN_DURATION_IN_SEC,
            turns_counter: 0,
        };

        app.insert_resource(turn_timer);
        //.add_systems(Update, update_turn);
    }
}
