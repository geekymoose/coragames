use crate::action::Action;
use crate::player::Player;
use crate::player::PlayerTurnRequest;
use crate::terrain::Config;
use crate::terrain::Grid;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Game<'a> {
    current_turn: u32,
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

    pub fn add_player(&mut self, id: u32, name: String) -> Result<&Player, &'static str> {
        // TODO Improve the errors (for now, just a simple WIP with strings)

        let unit = match self.gamegrid.spwan_random_unit() {
            Some(unit) => unit,
            None => return Err("Unable to spwan a new Unit for the player"),
        };
        let player = Player::new(id, name, &unit);

        if self.players.contains_key(&id) {
            return Err("Unable to add player: the ID already exists");
        } else {
            // TODO Fixme
            //self.players.insert(id, player);
            //return Ok(&player);
            todo!("WIP Not Implemented");
        }
    }

    pub fn request_turn_action(&mut self) {
        self.players.values_mut().for_each(|p: &mut Player| {
            p.request_turn_action(self.current_turn);
        });
    }

    pub fn register_player_response(
        &mut self,
        player_id: u32,
        action: Action,
    ) -> Result<bool, &'static str> {
        let player: &mut Player = match self.players.get_mut(&player_id) {
            Some(p) => p,
            None => return Err("The requested player doesn't exists in-game"),
        };

        return player.register_turn_action(self.current_turn, action);
    }

    pub fn apply_turn(&mut self) {
        // TODO The order in witch action are concluded should use a better approach (e.g., received first applied first)
        self.players.values_mut().for_each(|p: &mut Player| {
            p.apply_turn_action();
        });

        self.current_turn += 1;
    }
}
