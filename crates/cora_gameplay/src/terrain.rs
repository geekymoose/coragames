use crate::unit::Unit;

pub struct Config {
    pub width: u32,
    pub height: u32,
}

pub struct Grid {
    cells: Vec<Vec<Cell>>,
}

pub struct Cell {
    unit: Option<Unit>,
    terrain: EnvironmentType,
}

pub enum EnvironmentType {
    Movable,
    Impassable,
}
