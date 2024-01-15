pub struct Config {
    pub width: usize,
    pub height: usize,
}

pub(crate) struct Grid {
    cells: Vec<Vec<Cell>>,
}

pub(crate) struct Cell {
    terrain: EnvironmentType,
}

pub(crate) enum EnvironmentType {
    Movable,
    Impassable,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            terrain: EnvironmentType::Movable,
        }
    }
}

impl Grid {
    pub(crate) fn new(config: &Config) -> Self {
        todo!("Not Implemented");
    }
}
