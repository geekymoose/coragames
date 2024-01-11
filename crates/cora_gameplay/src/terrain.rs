use crate::unit::Unit;

pub(crate) struct Config {
    pub width: u32,
    pub height: u32,
}

pub(crate) struct Grid {
    cells: Vec<Vec<Cell>>,
}

pub(crate) struct Cell {
    unit: Option<Unit>,
    terrain: EnvironmentType,
}

pub(crate) enum EnvironmentType {
    Movable,
    Impassable,
}
