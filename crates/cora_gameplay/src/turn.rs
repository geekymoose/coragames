use std::collections::HashMap;

use crate::{
    action::{apply_action, Action},
    grid_map::Grid,
    unit::Unit,
    vision::GridVision,
};

#[derive(Debug)]
pub struct Turn {
    turn_counter: u32,
    turn_duration_in_ms: u32,
    turn_requests: HashMap<u32, TurnActionRequest>,
    turn_response: Vec<TurnActionResponse>,
}

#[derive(Debug)]
pub struct TurnActionRequest {
    turn_start: u32,
}

#[derive(Debug)]
pub struct TurnActionResponse {
    unit_id: u32,
    turn_start: u32,
    turn_end: u32,
    action: Action,
}

impl Turn {
    pub fn new(turn_duration_in_ms: u32) -> Self {
        Self {
            turn_counter: 0,
            turn_duration_in_ms,
            turn_requests: HashMap::new(),
            turn_response: Vec::new(),
        }
    }

    pub fn request_all_turn_actions(
        &mut self,
        units: &HashMap<u32, Unit>,
        grid: &Grid,
    ) -> HashMap<u32, GridVision> {
        let mut result: HashMap<u32, GridVision> = HashMap::with_capacity(units.len());

        for unit_bucket in units {
            let unit_id = unit_bucket.0;
            let unit = unit_bucket.1;

            if self.turn_requests.contains_key(unit_id) {
                // This unit is already processing a request
                continue;
            }

            let request = TurnActionRequest {
                turn_start: self.turn_counter,
            };

            self.turn_requests.insert(*unit_id, request);

            let vision = GridVision::new_vision_of(
                grid,
                unit.grid_unit().x(),
                unit.grid_unit().y(),
                unit.vision_range(),
            );

            result.insert(*unit_id, vision);
        }

        return result;
    }

    pub fn register_turn_action_response(
        &mut self,
        unit_id: u32,
        action: Action,
    ) -> Result<(), &'static str> {
        let request = match self.turn_requests.remove(&unit_id) {
            Some(unit) => unit,
            None => return Err(
                "Unable to register the action: no TurnActionRequest were registered for this unit",
            ),
        };

        let response = TurnActionResponse {
            unit_id,
            turn_start: request.turn_start,
            turn_end: self.turn_counter,
            action,
        };

        self.turn_response.push(response);
        return Ok(());
    }

    pub fn apply_all_turn_actions(&mut self, units: &HashMap<u32, Unit>, grid: &mut Grid) {
        for request in &self.turn_response {
            let unit = match units.get(&request.unit_id) {
                Some(unit) => unit,
                None => continue, // This unit maybe have been removed from the game since then
            };

            match apply_action(&request.action, unit, grid) {
                Ok(_) => continue,
                Err(msg) => println!("Error while applying the action: {}", msg),
            };
        }

        self.turn_response.clear();
    }

    pub fn next_turn(&mut self) {
        self.turn_counter += 1;
    }

    pub fn turn_duration_in_ms(&self) -> u32 {
        return self.turn_duration_in_ms;
    }

    pub fn current_turn(&self) -> u32 {
        return self.turn_counter;
    }
}
