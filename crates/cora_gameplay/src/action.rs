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
