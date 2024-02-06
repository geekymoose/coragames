use crate::{direction::Direction, grid_map::Grid, grid_unit::GridUnit};

pub(crate) fn move_unit(
    unit: GridUnit,
    grid: &mut Grid,
    direction: &Direction,
) -> Result<GridUnit, &'static str> {
    let dest = match grid.neighbor_at_direction(unit.x(), unit.y(), direction) {
        Some(cell) => cell,
        None => return Err(
            "Unable to get the neighbor cell for this direction (you may have reach the border)",
        ),
    };

    if dest.has_unit() {
        return Err("Unable to move on a cell that already has a Unit on it");
    }
    if !dest.is_walkable() {
        return Err("Unable to move on a cell that is not walkable");
    }

    let src = match grid.cell_at_pos_mut(unit.x(), unit.y()) {
        Some(cell) => cell,
        None => return Err("Unable to get the requested cell from the Grid"),
    };

    let unit = match src.remove_unit() {
        Some(unit) => unit,
        None => {
            return Err("Internal error: the src Unit should have the requested Unit before moving")
        }
    };

    let dest = match grid.neighbor_at_direction_mut(unit.x(), unit.y(), direction) {
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
