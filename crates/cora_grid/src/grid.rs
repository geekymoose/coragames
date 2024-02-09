use crate::{config::GridConfig, coordinate::GridCoordinate, direction::GridDirection};

#[derive(Debug)]
pub(crate) struct Grid<T: Default> {
    config: GridConfig,
    cells: Vec<T>,
}

impl<T: Default> Grid<T> {
    pub fn new(config: GridConfig) -> Self {
        let mut cells = Vec::with_capacity(config.size());
        for _i in 0..config.size() {
            cells.push(T::default());
        }
        return Grid { config, cells };
    }

    pub fn size(&self) -> usize {
        return self.cells.len();
    }

    pub fn width(&self) -> usize {
        return self.config.width();
    }

    pub fn height(&self) -> usize {
        return self.config.height();
    }

    pub fn get(&self, coordinates: &GridCoordinate) -> Option<&T> {
        if !self.is_valid_coordinates(coordinates) {
            return None;
        }
        let index = self.coordinates_to_grid_index(coordinates);
        return match &self.cells.get(index) {
            Some(elt) => Some(&elt),
            None => None,
        };
    }

    pub fn get_mut(&mut self, coordinates: &GridCoordinate) -> Option<&mut T> {
        if !self.is_valid_coordinates(coordinates) {
            return None;
        }
        let index = self.coordinates_to_grid_index(coordinates);
        let cell = &mut self.cells[index];
        return Some(cell);
    }

    pub fn get_neighbor_at_direction(
        &self,
        coordinates: &GridCoordinate,
        direction: &GridDirection,
    ) -> Option<&T> {
        return match coordinates.get_neighbor_coordinates_at_direction(&self.config, &direction) {
            Some(neighbor) => self.get(&neighbor),
            None => None,
        };
    }

    pub fn get_neighbor_at_direction_mut(
        &mut self,
        coordinates: &GridCoordinate,
        direction: &GridDirection,
    ) -> Option<&mut T> {
        return match coordinates.get_neighbor_coordinates_at_direction(&self.config, &direction) {
            Some(neighbor) => self.get_mut(&neighbor),
            None => None,
        };
    }

    pub fn is_valid_coordinates(&self, coordinates: &GridCoordinate) -> bool {
        return coordinates.is_valid_in_grid(&self.config);
    }

    pub fn random_coordinates(&self) -> GridCoordinate {
        return GridCoordinate::new_random(&self.config);
    }

    fn coordinates_to_grid_index(&self, coordinates: &GridCoordinate) -> usize {
        return coordinates.x() + (coordinates.y() * self.width());
    }
}
