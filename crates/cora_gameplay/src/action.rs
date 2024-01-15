use crate::{terrain::Grid, unit::Unit};

pub(crate) enum Action {
    Move(Direction),
    Attack(Direction),
    Interact(Direction),
}

pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub(crate) fn apply_action(action: Action, location: &mut Grid, unit: &mut Unit) {
    match action {
        Action::Move(direction) => unit.movement(direction),
        Action::Attack(direction) => unit.attack(direction),
        Action::Interact(direction) => unit.interact(direction),
    }
}
