use crate::{terrain::Direction, unit::Unit};

#[derive(Debug)]
pub(crate) enum Action {
    Move(Direction),
    Attack(Direction),
    Interact(Direction),
}

pub(crate) fn apply_action(action: Action, unit: &mut Unit) {
    match action {
        Action::Move(direction) => unit.movement(direction),
        Action::Attack(direction) => unit.attack(direction),
        Action::Interact(direction) => unit.interact(direction),
    }
}
