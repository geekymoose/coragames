use crate::grid_map::Grid;

#[derive(Debug)]
pub struct GridVision {
    vision_range: usize,
    vision_width: usize,
    vision_height: usize,
    vision_grid: Vec<GridVisionData>,
}

#[derive(Debug)]
pub struct GridVisionData {
    movable: bool,
    unit: bool,
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

    pub(crate) fn new_vision_of(
        grid: &Grid,
        x_in_grid: usize,
        y_in_grid: usize,
        vision_range: usize,
    ) -> Self {
        let mut vision = GridVision::new(vision_range);

        let x_start = x_in_grid - vision.vision_range;
        let x_end = x_in_grid + vision_range;
        let y_start = y_in_grid - vision_range;
        let y_end = y_in_grid + vision_range;

        let x_range = x_start..(x_end + 1);
        let y_range = (y_start..(y_end + 1)).rev();

        for y in y_range {
            for x in x_range.clone() {
                match grid.cell_at_pos(x, y) {
                    Some(cell) => {
                        let elt = GridVisionData {
                            movable: cell.is_walkable(),
                            unit: cell.has_unit(),
                        };
                        vision.vision_grid.push(elt);
                    }
                    None => {
                        // Means it's out of the border, we can't move there
                        let elt = GridVisionData {
                            movable: false,
                            unit: false,
                        };
                        vision.vision_grid.push(elt);
                    }
                }
            }
        }

        return vision;
    }
}
