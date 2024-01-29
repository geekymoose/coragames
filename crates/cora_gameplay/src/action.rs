use crate::{
    terrain::{self, Direction, Grid},
    unit::Unit,
};

#[derive(Debug)]
pub enum Action {
    Move(Direction),
    Attack(Direction),
    Interact(Direction),
}

pub(crate) fn apply_action(action: Action, unit: &mut Unit, terrain: &mut Grid) {
    match action {
        Action::Move(direction) => match terrain::move_unit(unit, terrain, direction) {
            Ok(_) => println!("Move action applied"),
            Err(msg) => println!("Unable to applie the move action: {}", msg),
        },
        Action::Attack(_) => todo!("Not Implemented"),
        Action::Interact(_) => todo!("Not Implemented"),
    }
}
