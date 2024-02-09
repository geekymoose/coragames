use crate::{grid_cell::GridCell, grid_map::SquareGrid2D};

pub fn get_random_cell_mut(
    grid: &mut SquareGrid2D<GridCell>,
    is_walkable: bool,
    has_unit: bool,
) -> Result<&mut GridCell, &'static str> {
    // WIP This is an absolute horrible implementation , just for the WIP
    let mut retries = 100;

    loop {
        let random = grid.random_coordinates();
        let cell = grid.get(&random).unwrap(); // unwrap: we know the generated is valid

        if cell.is_walkable() == is_walkable && cell.has_unit() == has_unit {
            let cell = grid.get_mut(&random).unwrap();
            return Ok(cell);
        } else {
            retries -= 1;
            if retries <= 0 {
                return Err("Unable to find a random cell that satisfy the prerequisits");
            }
        }
    }
}
