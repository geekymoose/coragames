use crate::{
    config::GridConfig,
    direction::Direction,
    grid_cell::GridCell,
    grid_coordinate::{is_valid_in_grid, neighbor_coordinates_at_direction, random_coordinates},
};

#[derive(Debug)]
pub(crate) struct Grid {
    config: GridConfig,
    cells: Vec<Vec<GridCell>>,
}

impl Grid {
    pub fn new(config: GridConfig) -> Self {
        let mut grid = Grid {
            config,
            cells: vec![vec![]],
        };
        grid.generate_random_cells();
        return grid;
    }

    pub fn cell_at_pos(&self, x: usize, y: usize) -> Option<&GridCell> {
        if self.is_valid_coordinates(x, y) {
            return Some(&self.cells[x][y]);
        }
        return None;
    }

    pub fn cell_at_pos_mut(&mut self, x: usize, y: usize) -> Option<&mut GridCell> {
        if self.is_valid_coordinates(x, y) {
            return Some(&mut self.cells[x][y]);
        }
        return None;
    }

    pub fn neighbor_at_direction(
        &self,
        x: usize,
        y: usize,
        direction: &Direction,
    ) -> Option<&GridCell> {
        if !self.is_valid_coordinates(x, y) {
            return None;
        }

        let new_coord_option = neighbor_coordinates_at_direction(&self.config, x, y, &direction);

        return match new_coord_option {
            Some(new_coord) => self.cell_at_pos(new_coord.x(), new_coord.y()),
            None => None,
        };
    }

    pub fn neighbor_at_direction_mut(
        &mut self,
        x: usize,
        y: usize,
        direction: &Direction,
    ) -> Option<&mut GridCell> {
        if !self.is_valid_coordinates(x, y) {
            return None;
        }

        let new_coord_option = neighbor_coordinates_at_direction(&self.config, x, y, &direction);

        return match new_coord_option {
            Some(new_coord) => self.cell_at_pos_mut(new_coord.x(), new_coord.y()),
            None => None,
        };
    }

    pub fn get_random_cell_mut(
        &mut self,
        is_walkable: bool,
        has_unit: bool,
    ) -> Result<&mut GridCell, &'static str> {
        // WIP This is an absolute horrible implementation , just for the WIP
        let mut retries = 100;

        loop {
            let random = random_coordinates(&self.config);
            let cell = self.cell_at_pos(random.x(), random.y()).unwrap();

            if cell.is_walkable() == is_walkable && cell.has_unit() == has_unit {
                let cell = self.cell_at_pos_mut(random.x(), random.y()).unwrap();
                return Ok(cell);
            } else {
                retries -= 1;
                if retries <= 0 {
                    return Err("Unable to find a random cell that satisfy the prerequisits");
                }
            }
        }
    }

    fn generate_random_cells(&mut self) {
        self.cells.clear();
        self.cells.reserve(self.config.width);

        for x in 0..self.config.width {
            let mut column = vec![];
            column.reserve(self.config.height);
            self.cells.push(column);

            for y in 0..self.config.height {
                let cell = GridCell::new(x, y, true);
                self.cells[x].push(cell);
            }
        }
    }

    fn is_valid_coordinates(&self, x: usize, y: usize) -> bool {
        return is_valid_in_grid(&self.config, x, y);
    }
}
