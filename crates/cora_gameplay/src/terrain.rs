pub struct Config {
    pub width: usize,
    pub height: usize,
}

pub(crate) struct Grid<'a> {
    config: Config,
    cells: Vec<Vec<Cell<'a>>>,
}

pub(crate) struct Cell<'a> {
    x: usize,
    y: usize,
    grid: &'a Grid<'a>,
    terrain_type: EnvironmentType,
}

pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub(crate) enum EnvironmentType {
    Movable,
    Impassable,
}

impl<'a> Grid<'a> {
    pub(crate) fn new(config: Config) -> Self {
        todo!("Not Implemented");
    }
}

impl<'a> Cell<'a> {
    pub(crate) fn new(x: usize, y: usize, grid: &'a Grid, terrain_type: EnvironmentType) -> Self {
        Self {
            x,
            y,
            grid,
            terrain_type,
        }
    }

    pub(crate) fn neighbor_at_direction(&self, direction: &Direction) -> Option<Cell<'a>> {
        todo!("Not Implemented");
    }
}
