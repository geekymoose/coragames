use crate::action::Action;
use crate::player::Player;
use crate::terrain::Config;
use crate::terrain::Grid;
use std::collections::HashMap;

// Used for the unique player IDs (it's not the best way but OK for now)
static ID_COUNTER: u32 = 0;

pub struct Game<'a> {
    current_turn: usize,
    gamegrid: Grid,
    players: HashMap<u32, Player<'a>>,
}

impl<'a> Game<'a> {
    pub fn new(config: &Config) -> Self {
        Self {
            current_turn: 0,
            gamegrid: Grid::new(config),
            players: HashMap::new(),
        }
    }

    pub fn register_player_action_turn(player: &mut Player, player_action: Action) {
        todo!("Not Implemented");
    }

    pub fn apply_turn(&mut self) {
        self.players.values_mut().for_each(|p: &mut Player| {
            p.apply_turn_action();
        });

        self.current_turn += 1;
    }
}
