use crate::terrain::Direction;

#[derive(Debug)]
pub enum Action {
    Move(Direction),
    Attack(Direction),
    Interact(Direction),
}

pub(crate) fn apply_action(action: Action) {
    match action {
        Action::Move(_) => todo!("Not Implemented"),
        Action::Attack(_) => todo!("Not Implemented"),
        Action::Interact(_) => todo!("Not Implemented"),
    }
}
