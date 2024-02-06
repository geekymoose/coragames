use crate::{
    combat::{Health, Weapon},
    grid_unit::GridUnit,
};

#[derive(Debug)]
pub struct Unit {
    id: u32,
    grid_unit: GridUnit,
    health: Health,
    weapon: Weapon,
    vision_range: usize,
}

impl Unit {
    pub fn new(
        id: u32,
        grid_unit: GridUnit,
        health: Health,
        weapon: Weapon,
        vision: usize,
    ) -> Self {
        Self {
            id,
            grid_unit,
            health,
            weapon,
            vision_range: vision,
        }
    }
}
