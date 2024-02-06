use crate::combat::{Health, Weapon};
use crate::config::*;
use crate::grid_map::Grid;
use crate::player::Player;
use crate::spawn::spwan_random_unit_in_grid;
use crate::unit::Unit;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Game {
    gamegrid: Grid,
    players: HashMap<u32, Player>,
}

impl Game {
    pub fn new(config: GridConfig) -> Self {
        Self {
            gamegrid: Grid::new(config),
            players: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, id: u32, name: String) -> Result<(), &'static str> {
        let grid_unit = match spwan_random_unit_in_grid(&mut self.gamegrid, id) {
            Ok(unit) => unit,
            Err(msg) => return Err(msg),
        };
        let health = Health::new(DEFAULT_UNIT_HEALTH);
        let weapon = Weapon::new(DEFAULT_UNIT_STRENGTH);
        let unit = Unit::new(
            id,
            name.clone(),
            grid_unit,
            health,
            weapon,
            DEFAULT_UNIT_VISION_RANGE,
        );
        let player = Player::new(id, name, unit);

        match self.players.entry(id) {
            std::collections::hash_map::Entry::Occupied(_) => {
                return Err("Unable to add player: the ID already exists");
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(player);
                return Ok(());
            }
        }
    }

    /*
    // TODO TMP to remove when this is fully re-integrated
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
    */
}
