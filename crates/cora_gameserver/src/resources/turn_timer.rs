use std::time::Duration;

use bevy::{
    ecs::system::Resource,
    time::{Timer, TimerMode},
};

#[derive(Resource)]
pub struct TurnTimer {
    timer: Timer,
    turn_duration_in_sec: f32,
    turns_counter: u32,
}

impl TurnTimer {
    pub fn new(duration_in_sec: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration_in_sec, TimerMode::Repeating),
            turn_duration_in_sec: duration_in_sec,
            turns_counter: 0,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        self.timer.tick(dt);
    }

    pub fn counter(&self) -> u32 {
        return self.turns_counter;
    }

    pub fn turn_duration_in_sec(&self) -> f32 {
        return self.turn_duration_in_sec;
    }

    pub fn just_finished(&self) -> bool {
        return self.timer.just_finished();
    }

    pub fn reset(&mut self) {
        self.turns_counter = 0;
        self.timer.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_simple_usage() {
        let timer = TurnTimer::new(10.0); // Each 10 seconds

        assert_eq!(timer.counter(), 0);
        assert_eq!(timer.turn_duration_in_sec(), 10.0);
        assert_eq!(timer.just_finished(), false);
    }
}
