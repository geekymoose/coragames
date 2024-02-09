use crate::{
    grid_cell::GridCell, grid_map::SquareGrid2D, grid_unit::GridUnit, world::get_random_cell_mut,
};

pub fn spwan_random_unit_in_grid(
    grid: &mut SquareGrid2D<GridCell>,
    unit_id: u32,
) -> Result<GridUnit, &'static str> {
    let cell: &mut GridCell = match get_random_cell_mut(grid, true, false) {
        Ok(found_cell) => found_cell,
        Err(msg) => return Err(msg),
    };

    let unit = GridUnit::new(unit_id);

    return match cell.place_unit(unit) {
        Ok(unit) => Ok(unit),
        Err(msg) => Err(msg),
    };
}
