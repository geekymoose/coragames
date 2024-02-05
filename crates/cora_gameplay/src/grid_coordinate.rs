use rand::Rng;

use crate::{config::GridConfig, direction::Direction};

#[derive(Debug, Default, Clone, Copy)]
pub struct GridCoordinate {
    x: usize,
    y: usize,
}

impl GridCoordinate {
    pub fn new(x: usize, y: usize) -> Self {
        return Self { x: x, y: y };
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

    pub fn is_in_grid(&self, config: &GridConfig) -> bool {
        return is_valid_in_grid(config, self.x, self.y);
    }
}

pub fn is_valid_in_grid(config: &GridConfig, x: usize, y: usize) -> bool {
    let valid_x = x < config.width;
    let valid_y = y < config.height;
    return valid_x && valid_y;
}

pub fn neighbor_coordinates_at_direction(
    config: &GridConfig,
    x: usize,
    y: usize,
    direction: &Direction,
) -> Option<GridCoordinate> {
    if !is_valid_in_grid(config, x, y) {
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

    return Some(GridCoordinate { x: dir_x, y: dir_y });
}

pub fn random_coordinates(config: &GridConfig) -> GridCoordinate {
    let mut rng = rand::thread_rng();

    let rand_x = rng.gen_range(0..config.width);
    let rand_y = rng.gen_range(0..config.height);

    return GridCoordinate {
        x: rand_x,
        y: rand_y,
    };
}
