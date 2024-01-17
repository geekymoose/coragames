use crate::{terrain::Direction, unit::Unit};

#[derive(Debug)]
pub(crate) enum Action {
    Move(Direction),
    Attack(Direction),
    Interact(Direction),
}

pub(crate) fn apply_action(action: Action, unit: &mut Unit) {
    match action {
        Action::Move(direction) => todo!("Not Implemented"),
        Action::Attack(direction) => todo!("Not Implemented"),
        Action::Interact(direction) => todo!("Not Implemented"),
    }
}
