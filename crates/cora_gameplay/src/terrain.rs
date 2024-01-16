#[derive(Debug)]
pub struct Config {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug)]
pub(crate) struct Grid {
    config: Config,
    cells: Vec<Vec<Cell>>,
}

#[derive(Debug)]
pub(crate) struct Cell {
    x: usize,
    y: usize,
    terrain_type: EnvironmentType,
}

#[derive(Debug)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
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
            self.cells[x].reserve(self.config.height);
            for y in 0..self.config.height {
                let cell = Cell::new(x, y, EnvironmentType::Movable);
                self.cells[x].push(cell);
            }
        }
    }

    fn get_cell_at_pos(&self, x: usize, y: usize) -> Option<&Cell> {
        if self.is_valid_coordinates(x, y) {
            return Some(&self.cells[x][y]);
        }
        return None;
    }

    fn get_neighbor_at_direction(&self, x: usize, y: usize, direction: Direction) -> Option<&Cell> {
        if !self.is_valid_coordinates(x, y) {
            return None;
        }

        let mut dir_x = x;
        let mut dir_y = y;

        match direction {
            Direction::Up => {
                dir_y += 1;
            }
            Direction::Down => {
                dir_y -= 1;
            }
            Direction::Left => {
                dir_x -= 1;
            }
            Direction::Right => {
                dir_x += 1;
            }
        }

        return self.get_cell_at_pos(dir_x, dir_y);
    }

    fn is_valid_coordinates(&self, x: usize, y: usize) -> bool {
        let valid_x = x < self.config.width;
        let valid_y = y < self.config.height;
        return valid_x && valid_y;
    }
}

impl Cell {
    pub(crate) fn new(x: usize, y: usize, terrain_type: EnvironmentType) -> Self {
        Self { x, y, terrain_type }
    }
}
