use crate::grid_unit::GridUnit;

#[derive(Debug)]
pub struct GridCell {
    walkable: bool,
    unit: Option<GridUnit>,
}

impl GridCell {
    pub fn is_walkable(&self) -> bool {
        return self.walkable;
    }

    pub fn has_unit(&self) -> bool {
        return match self.unit {
            Some(_) => true,
            None => false,
        };
    }

    pub fn place_unit(&mut self, unit: GridUnit) -> Result<GridUnit, &'static str> {
        if self.has_unit() {
            return Err("The cell already has a Unit");
        } else {
            self.unit = Some(unit);
            return Ok(unit);
        }
    }

    pub fn remove_unit(&mut self) -> Option<GridUnit> {
        return self.unit.take();
    }
}

impl Default for GridCell {
    fn default() -> Self {
        Self {
            walkable: true,
            unit: None,
        }
    }
}
