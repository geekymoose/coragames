use crate::action::Action;
use crate::player::Player;
use crate::terrain::Config;
use crate::terrain::Grid;
use std::collections::HashMap;

pub struct Game<'a> {
    current_turn: usize,
    gamegrid: Grid,
    players: HashMap<u32, Player<'a>>,
}

impl<'a> Game<'a> {
    pub fn new(config: Config) -> Self {
        Self {
            current_turn: 0,
            gamegrid: Grid::new(config),
            players: HashMap::new(),
        }
    }

    pub fn register_player_action_turn(&mut self, player_id: &u32, player_action: Action) {
        todo!("Not Implemented");
    }

    pub fn apply_turn(&mut self) {
        // TODO The order in witch action are concluded should use a better approach (e.g., received first applied first)
        self.players.values_mut().for_each(|p: &mut Player| {
            p.apply_turn_action();
        });

        self.current_turn += 1;
    }
}
