pub struct Config {
    pub width: usize,
    pub height: usize,
}

pub(crate) struct Grid {
    config: Config,
    cells: Vec<Vec<Cell>>,
}

pub(crate) struct Cell {
    x: usize,
    y: usize,
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

impl Grid {
    pub(crate) fn new(config: Config) -> Self {
        let mut grid = Grid {
            config,
            cells: vec![vec![]],
        };
        grid.generate_random_cells();
        return grid;
    }

    fn generate_random_cells(&mut self) {
        self.cells.clear();
        self.cells.reserve(self.config.width);

        for x in 0..self.config.width {
            self.cells.reserve(self.config.height);
            for y in 0..self.config.height {
                let cell = Cell::new(x, y, EnvironmentType::Movable);
                self.cells[x].push(cell);
            }
        }
    }
}

impl Cell {
    pub(crate) fn new(x: usize, y: usize, terrain_type: EnvironmentType) -> Self {
        Self { x, y, terrain_type }
    }
}
