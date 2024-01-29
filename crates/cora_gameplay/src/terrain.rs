use rand::Rng;

use crate::unit::Unit;

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
    unit: Option<Unit>,
}

#[derive(Debug)]
pub enum Direction {
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

pub(crate) fn move_unit(
    unit: &mut Unit,
    terrain: &mut Grid,
    direction: Direction,
) -> Result<(), &'static str> {
    let x = unit.pos_world_x;
    let y = unit.pos_world_y;

    let dest_coord = match terrain.get_neighbor_coord_at_direction(x, y, &direction) {
        Some(coord) => coord,
        None => return Err("The requested move has reach the border and cannot be applied"),
    };

    if (!terrain.is_cell_free_at_coord(dest_coord.0, dest_coord.1)) {
        return Err("Cannot move the Unit, the dest cell has already a Unit");
    }

    let dest_cell = match terrain.get_mut_neighbor_at_direction(x, y, direction) {
        Some(value) => value,
        None => return Err("Unable to get the destination cell where to move the unit"),
    };

    let unit = match terrain.remove_unit_from_coord(x, y) {
        Ok(unit_option) => match unit_option {
            Some(value) => value,
            None => return Err("The grid cell has no unit at this position"),
        },
        Err(msg) => return Err(msg),
    };
    todo!("WIP");
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
            let mut column = vec![];
            column.reserve(self.config.height);
            self.cells.push(column);

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

    fn get_mut_cell_at_pos(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if self.is_valid_coordinates(x, y) {
            return Some(&mut self.cells[x][y]);
        }
        return None;
    }

    fn get_neighbor_at_direction(&self, x: usize, y: usize, direction: Direction) -> Option<&Cell> {
        if !self.is_valid_coordinates(x, y) {
            return None;
        }

        let new_coord_option = self.get_neighbor_coord_at_direction(x, y, &direction);

        return match new_coord_option {
            Some(new_coord) => self.get_cell_at_pos(new_coord.0, new_coord.1),
            None => None,
        };
    }

    pub fn get_mut_neighbor_at_direction(
        &mut self,
        x: usize,
        y: usize,
        direction: Direction,
    ) -> Option<&mut Cell> {
        if !self.is_valid_coordinates(x, y) {
            return None;
        }

        let new_coord_option = self.get_neighbor_coord_at_direction(x, y, &direction);

        return match new_coord_option {
            Some(new_coord) => self.get_mut_cell_at_pos(new_coord.0, new_coord.1),
            None => None,
        };
    }

    pub fn get_neighbor_coord_at_direction(
        &self,
        x: usize,
        y: usize,
        direction: &Direction,
    ) -> Option<(usize, usize)> {
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

        return Some((dir_x, dir_y));
    }

    fn is_valid_coordinates(&self, x: usize, y: usize) -> bool {
        let valid_x = x < self.config.width;
        let valid_y = y < self.config.height;
        return valid_x && valid_y;
    }

    fn get_random_movable_cell(&self) -> Option<&Cell> {
        // WIP This is an absolute horrible implementation , just for the WIP
        let mut retries = 100;

        loop {
            let mut rng = rand::thread_rng();

            let rand_x = rng.gen_range(0..self.config.width);
            let rand_y = rng.gen_range(0..self.config.height);

            let cell = &self.cells[rand_x][rand_y];
            match cell.terrain_type {
                EnvironmentType::Movable => {
                    return Some(&cell);
                }
                EnvironmentType::Impassable => {
                    retries -= 1;
                    if retries <= 0 {
                        return None;
                    }
                }
            }
        }
    }

    fn move_unit(
        &mut self,
        unit: &mut Unit,
        origin: &mut Cell,
        dest: &mut Cell,
    ) -> Result<(), &'static str> {
        unit.movement(dest.x, dest.y);
        dest.unit = origin.unit.take();
        return Ok(());
    }

    fn is_cell_free(&self, cell: &Cell) -> bool {
        return match cell.unit {
            Some(_) => false,
            None => true,
        };
    }

    fn is_cell_free_at_coord(&self, x: usize, y: usize) -> bool {
        let cell = match self.get_cell_at_pos(x, y) {
            Some(value) => value,
            None => return false,
        };
        return self.is_cell_free(cell);
    }

    fn place_unit_at_cell(&mut self, mut unit: Unit, cell: &mut Cell) -> Result<(), Unit> {
        match cell.unit {
            Some(_) => return Err(unit),
            None => {
                unit.movement(cell.x(), cell.y());
                cell.unit = Some(unit)
            }
        }
        return Ok(());
    }

    fn place_unit_at_coord(&mut self, mut unit: Unit, x: usize, y: usize) -> Result<(), Unit> {
        match self.get_mut_cell_at_pos(x, y) {
            Some(cell) => {
                cell.unit = Some(unit);
                return Ok(());
            }
            None => return Err(unit),
        };
    }

    fn remove_unit_from_cell(&mut self, cell: &mut Cell) -> Result<Option<Unit>, &'static str> {
        match &cell.unit {
            Some(unit) => return Ok(cell.unit.take()),
            None => return Ok(None),
        }
    }

    fn remove_unit_from_coord(&mut self, x: usize, y: usize) -> Result<Option<Unit>, &'static str> {
        return match self.get_mut_cell_at_pos(x, y) {
            Some(value) => Ok(value.unit.take()),
            None => Err("Unable to remove, the coordinates are invalid"),
        };
    }

    pub(crate) fn spwan_random_unit(&mut self) -> Option<&Unit> {
        let spawn_cell = match self.get_random_movable_cell() {
            Some(found_cell) => found_cell,
            None => return None,
        };

        let spawn_cell: &mut Cell = match self.get_mut_cell_at_pos(spawn_cell.x(), spawn_cell.y()) {
            Some(cell) => cell,
            None => return None,
        };

        let spawn_unit = Unit::new_default(spawn_cell.x(), spawn_cell.y());

        spawn_cell.unit = Some(spawn_unit);
        return match &spawn_cell.unit {
            Some(unit_cell) => Some(&unit_cell),
            None => None,
        };
    }
}

impl Cell {
    pub(crate) fn new(x: usize, y: usize, terrain_type: EnvironmentType) -> Self {
        Self {
            x,
            y,
            terrain_type,
            unit: None,
        }
    }

    pub(crate) fn x(&self) -> usize {
        self.x
    }

    pub(crate) fn y(&self) -> usize {
        self.y
    }
}
