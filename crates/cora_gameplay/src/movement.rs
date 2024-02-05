use crate::{direction::Direction, grid_map::Grid, grid_unit::GridUnit};

pub(crate) fn move_unit(
    unit: GridUnit,
    grid: &mut Grid,
    direction: Direction,
) -> Result<GridUnit, &'static str> {
    todo!("WIP Not Implemented");
    /*
    let dest_coord = match terrain.get_neighbor_coord_at_direction(x, y, &direction) {
        Some(coord) => coord,
        None => return Err("The requested move has reach the border and cannot be applied"),
    };

    if (!terrain.is_cell_free_at_coord(dest_coord.x, dest_coord.y)) {
        return Err("Cannot move the Unit, the dest cell has already a Unit");
    }

    let dest_cell = match terrain.get_mut_neighbor_at_direction(x, y, direction) {
        Some(value) => value,
        None => return Err("Unable to get the destination cell where to move the unit"),
    };

    let dest_x = dest_cell.x();
    let dest_y = dest_cell.y();

    match terrain.remove_unit_from_coord(dest_x, dest_y) {
        Ok(unit_option) => match unit_option {
            Some(value) => value,
            None => return Err("The grid cell has no unit at this position"),
        },
        Err(msg) => return Err(msg),
    };
    return Ok(());

    */
}
