use crate::{grid_cell::GridCell, grid_map::Grid, grid_unit::GridUnit};

pub fn spwan_random_unit_in_grid(grid: &mut Grid, unit_id: u32) -> Result<GridUnit, &'static str> {
    let cell: &mut GridCell = match grid.get_random_cell_mut(true, false) {
        Ok(found_cell) => found_cell,
        Err(msg) => return Err(msg),
    };

    let unit = GridUnit::new(unit_id);

    return match cell.place_unit(unit) {
        Ok(unit) => Ok(unit),
        Err(msg) => Err(msg),
    };
}
