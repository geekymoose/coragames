use rand::Rng;

use crate::{config::GridConfig, direction::GridDirection};

#[derive(Debug, Default, Clone, Copy)]
pub struct GridCoordinate {
    x: usize,
    y: usize,
}

impl GridCoordinate {
    pub fn new(x: usize, y: usize) -> Self {
        return Self { x: x, y: y };
    }

    pub fn new_random(config: &GridConfig) -> Self {
        let mut rng = rand::thread_rng();
        return Self {
            x: rng.gen_range(0..config.width()),
            y: rng.gen_range(0..config.height()),
        };
    }

    pub fn x(&self) -> usize {
        return self.x;
    }

    pub fn y(&self) -> usize {
        return self.y;
    }

    pub fn set(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn is_valid_in_grid(&self, config: &GridConfig) -> bool {
        let valid_x = self.x < config.width();
        let valid_y = self.y < config.height();
        return valid_x && valid_y;
    }

    pub fn get_neighbor_coordinates_at_direction(
        &self,
        config: &GridConfig,
        direction: &GridDirection,
    ) -> Option<GridCoordinate> {
        let mut neighbor = self.clone();

        match direction {
            GridDirection::Up => {
                neighbor.y += 1;
            }
            GridDirection::Down => {
                neighbor.y -= 1;
            }
            GridDirection::Left => {
                neighbor.x -= 1;
            }
            GridDirection::Right => {
                neighbor.x += 1;
            }
        }

        if !neighbor.is_valid_in_grid(config) {
            return None;
        }
        return Some(neighbor);
    }
}
