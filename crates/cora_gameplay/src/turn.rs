use std::{collections::HashMap, rc::Rc};

use crate::{action::Action, vision::GridVision};

#[derive(Debug)]
pub struct Turn {
    turn_counter: u32,
    turn_duration_in_ms: u32,
    turn_requests: HashMap<u32, TurnActionRequestStatus>,
}

#[derive(Debug)]
pub enum TurnActionRequestStatus {
    Idle,
    Computing(TurnActionRequest),
    Responded(TurnActionResponse),
}

#[derive(Debug)]
pub struct TurnActionRequest {
    unit_id: u32,
    turn_start: u32,
    vision: Rc<GridVision>,
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
        }
    }

    pub fn request_all_turn_actions(&self) -> Vec<TurnActionRequest> {
        let mut result: Vec<TurnActionRequest> = Vec::with_capacity(self.turn_requests.len());

        for unit_requests_status in self.turn_requests.iter() {
            match unit_requests_status.1 {
                TurnActionRequestStatus::Idle => {
                    // TODO Add vision
                    // TODO let request = todo!("WIP");
                    // TODO result.push(request)
                }
                TurnActionRequestStatus::Computing(_) => continue,
                TurnActionRequestStatus::Responded(_) => continue,
            }
        }

        return result;
    }

    pub fn register_turn_action_response(
        &mut self,
        unit_id: u32,
        action: Action,
    ) -> Result<(), &'static str> {
        todo!("WIP Not Implemented");
    }

    pub fn apply_all_turn_actions(&mut self) {
        for unit_requests_status in self.turn_requests.iter() {
            match unit_requests_status.1 {
                TurnActionRequestStatus::Idle => continue,
                TurnActionRequestStatus::Computing(_) => continue,
                TurnActionRequestStatus::Responded(response) => {
                    //self.turn_requests[unit_requests_status.0] = TurnActionRequestStatus::Idle;
                    todo!("WIP Not Implemented");
                }
            }
        }
    }

    pub fn next_turn(&mut self) {
        self.turn_counter += 1;
    }

    pub fn turn_duration_in_ms(&self) -> u32 {
        return self.turn_duration_in_ms;
    }
}

impl TurnActionRequest {
    pub fn unit_id(&self) -> u32 {
        return self.unit_id;
    }
}
