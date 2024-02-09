use crate::{
    direction::Direction, grid_cell::GridCell, grid_map::SquareGrid2D, grid_unit::GridUnit,
};

pub(crate) fn move_unit_on_grid(
    unit: GridUnit,
    grid: &mut SquareGrid2D<GridCell>,
    direction: &Direction,
) -> Result<GridUnit, &'static str> {
    // TODO CRITICAL Update the unit position data (until then, the unit does not actually move)
    let dest = match grid.get_neighbor_at_direction(&unit.grid_coordinates(), &direction) {
        Some(cell) => cell,
        None => return Err(
            "Unable to get the neighbor cell for this direction (you may have reach the border)",
        ),
    };

    if dest.has_unit() {
        return Err("Unable to move on a cell that already has a Unit on it");
    } else if !dest.is_walkable() {
        return Err("Unable to move on a cell that is not walkable");
    }

    let src = match grid.get_mut(&unit.grid_coordinates()) {
        Some(cell) => cell,
        None => return Err("Unable to get the requested cell from the Grid"),
    };

    let unit = match src.remove_unit() {
        Some(unit) => unit,
        None => {
            return Err("Internal error: the src Unit should have the requested Unit before moving")
        }
    };

    let dest = match grid.get_neighbor_at_direction_mut(&unit.grid_coordinates(), direction) {
        Some(cell) => cell,
        None => return Err(
            "Unable to get the neighbor cell for this direction (you may have reach the border)",
        ),
    };

    match dest.place_unit(unit) {
        Ok(unit) => return Ok(unit),
        Err(msg) => return Err(msg),
    }
}
