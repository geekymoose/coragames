use crate::grid_unit::GridUnit;

#[derive(Debug, Default)]
pub struct GridCell {
    x: usize,
    y: usize,
    walkable: bool,
    unit: Option<GridUnit>,
}

impl GridCell {
    pub fn new(x: usize, y: usize, walkable: bool) -> Self {
        Self {
            x,
            y,
            walkable,
            unit: None,
        }
    }

    pub fn x(&self) -> usize {
        return self.x;
    }

    pub fn y(&self) -> usize {
        return self.y;
    }

    pub fn is_walkable(&self) -> bool {
        return self.walkable;
    }

    pub fn has_unit(&self) -> bool {
        return match self.unit {
            Some(_) => true,
            None => false,
        };
    }

    pub fn unit(&self) -> Option<GridUnit> {
        return match self.unit {
            Some(unit) => Some(unit),
            None => None,
        };
    }

    pub fn place_unit(&mut self, mut unit: GridUnit) -> Result<GridUnit, &'static str> {
        if self.has_unit() {
            return Err("The cell already has a Unit");
        } else {
            unit.set_position(self.x(), self.y());
            self.unit = Some(unit);
            return Ok(unit);
        }
    }

    pub fn remove_unit(&mut self) -> Option<GridUnit> {
        return self.unit.take();
    }
}
