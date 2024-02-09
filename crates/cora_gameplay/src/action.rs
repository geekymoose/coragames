use crate::{
    direction::Direction, grid_cell::GridCell, grid_map::SquareGrid2D, grid_unit::GridUnit,
    movement, unit::Unit,
};

#[derive(Debug)]
pub enum Action {
    Move(Direction),
    Attack(Direction),
    Interact(Direction),
}

pub(crate) fn apply_action(
    action: &Action,
    unit: &Unit,
    grid: &mut SquareGrid2D<GridCell>,
) -> Result<GridUnit, &'static str> {
    match action {
        Action::Move(direction) => {
            return movement::move_unit_on_grid(*unit.grid_unit(), grid, &direction)
        }
        Action::Attack(_) => todo!("Not Implemented"),
        Action::Interact(_) => todo!("Not Implemented"),
    }
}
