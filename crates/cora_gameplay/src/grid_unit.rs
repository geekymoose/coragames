use crate::grid_coordinate::GridCoordinate;

#[derive(Debug, Clone, Copy)]
pub struct GridUnit {
    id: u32,
    coordinates: GridCoordinate,
}

impl GridUnit {
    pub fn new(id: u32) -> Self {
        return Self {
            id: id,
            coordinates: GridCoordinate::new(0, 0),
        };
    }

    pub fn id(&self) -> u32 {
        return self.id;
    }

    pub fn grid_coordinates(&self) -> GridCoordinate {
        return self.coordinates.clone();
    }

    pub fn set_position(&mut self, coordinates: &GridCoordinate) {
        self.coordinates = coordinates.clone();
    }
}
