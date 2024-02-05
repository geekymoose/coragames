use bevy::app::{App, Plugin, Update};

use crate::{resources::turn_timer::TurnTimer, systems::game_turns::update_turn};

const TURN_DURATION_IN_SEC: f32 = 1.0;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TurnTimer::new(TURN_DURATION_IN_SEC));
        app.add_systems(Update, update_turn);
    }
}
