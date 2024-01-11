use crate::action::Action;
use crate::{player::Player, terrain::Grid};
use std::collections::HashMap;

// Used for the unique player IDs (it's not the best way but OK for now)
static ID_COUNTER: u32 = 0;

pub struct Game<'a> {
    current_turn: usize,
    gamegrid: Grid,
    players: HashMap<u32, Player<'a>>,
}

impl<'a> Game<'a> {
    pub fn register_player_action_turn(player: &mut Player, player_action: Action) {
        player.action = Some(player_action);
    }

    pub fn apply_turn(&mut self) {
        self.players.values_mut().for_each(|p: &mut Player| {
            p.stats.update_played_one_turn(&p.action);

            match p.action.take() {
                Some(action) => match action {
                    Action::Move(Direction) => todo!(),
                    Action::Attack(Direction) => todo!(),
                    Action::Interact(Direction) => todo!(),
                },
                None => {
                    // The player did not requested any action
                    // Too bad for them
                }
            }
        });

        self.current_turn += 1;
    }
}
