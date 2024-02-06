use std::{collections::HashMap, rc::Rc};

use crate::{action::Action, vision::GridVision};

#[derive(Debug)]
pub struct TurnInfo {
    turn_counter: u32,
    turn_duration: u32,
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
    turn_start: u32,
    vision: Rc<GridVision>,
}

#[derive(Debug)]
pub struct TurnActionResponse {
    turn_start: u32,
    turn_end: u32,
    action: Action,
}
