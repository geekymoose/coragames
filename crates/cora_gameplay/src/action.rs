pub enum Action {
    Move(Direction),
    Attack(Direction),
    Interact(Direction),
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
