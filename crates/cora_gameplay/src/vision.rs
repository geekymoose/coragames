use crate::terrain::EnvironmentType;

#[derive(Debug)]
pub struct GridVision {
    vision_range: usize,
    vision_width: usize,
    vision_height: usize,
    vision_grid: Vec<GridVisionData>,
}

#[derive(Debug)]
pub struct GridVisionData {
    terrain_type: EnvironmentType,
    unit: u32,
}

impl GridVision {
    pub fn new(vision_range: usize) -> Self {
        // The vision grid is a square that includes the unit position + the range in all directions (width and height)
        let row_size = (vision_range * 2) + 1;
        let column_size = row_size;
        let vision_grid = Vec::with_capacity(row_size * column_size);

        Self {
            vision_range,
            vision_width: row_size,
            vision_height: column_size,
            vision_grid: vision_grid,
        }
    }
}
