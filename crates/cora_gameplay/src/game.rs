use crate::action::Action;
use crate::combat::{Health, Weapon};
use crate::config::*;
use crate::grid_map::Grid;
use crate::spawn::spwan_random_unit_in_grid;
use crate::turn::Turn;
use crate::unit::Unit;
use crate::vision::GridVision;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Game {
    turn: Turn,
    gamegrid: Grid,
    units: HashMap<u32, Unit>,
}

impl Game {
    pub fn new(config: GridConfig) -> Self {
        Self {
            turn: Turn::new(DEFAULT_TURN_DURACTION_IN_MS),
            gamegrid: Grid::new(config),
            units: HashMap::new(),
        }
    }

    pub fn add_unit(&mut self, id: u32, name: String) -> Result<(), &'static str> {
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

        match self.units.entry(id) {
            std::collections::hash_map::Entry::Occupied(_) => {
                return Err("Unable to add unit: the ID already exists");
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(unit);
                return Ok(());
            }
        }
    }

    pub fn remove_unit(&mut self, id: u32) -> Result<(), &'static str> {
        let unit = match self.units.remove(&id) {
            Some(unit) => unit,
            None => return Err("The requested Unit doesn exist in the game"),
        };

        let cell = match self
            .gamegrid
            .cell_at_pos_mut(unit.grid_unit().x(), unit.grid_unit().y())
        {
            Some(cell) => cell,
            None => return Err("Unable to find the unit in the grid"),
        };
        match cell.remove_unit() {
            Some(_) => return Ok(()),
            None => return Err("Error while removing the Unit from the grid"),
        }
    }

    pub fn units_count_in_game(&self) -> usize {
        return self.units.len();
    }

    pub fn has_unit(&self, id: u32) -> bool {
        return self.units.contains_key(&id);
    }

    pub fn request_turn_action(&mut self) -> HashMap<u32, GridVision> {
        return self
            .turn
            .request_all_turn_actions(&self.units, &self.gamegrid);
    }

    pub fn register_turn_action_response(
        &mut self,
        unit_id: u32,
        action: Action,
    ) -> Result<(), &'static str> {
        return self.turn.register_turn_action_response(unit_id, action);
    }

    pub fn apply_turn(&mut self) {
        self.turn.apply_all_turn_actions();
        self.turn.next_turn();
    }

    pub fn turn_duraction_in_ms(&self) -> u32 {
        return self.turn.turn_duration_in_ms();
    }

    pub fn current_turn(&self) -> u32 {
        return self.turn.current_turn();
    }
}
