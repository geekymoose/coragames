use crate::terrain::EnvironmentType;

/// Represents one unit currently ingame.
/// A unit is a something that can interact in the grid, move, attack.
/// It is meant to be controlled by a player or an AI.
#[derive(Debug)]
pub(crate) struct Unit {
    pos_world_x: usize,
    pos_world_y: usize,
    vision: UnitVision,
    health: u32,
    strength: u32,
    energy: u32,
}

#[derive(Debug)]
struct UnitVision {
    vision_range: usize,
    vision_width: usize,
    vision_height: usize,
    vision_grid: Vec<UnitVisionData>,
}

#[derive(Debug)]
struct UnitVisionData {
    terrain_type: EnvironmentType,
}

#[derive(Debug)]
struct DamageStat {
    effective_damage: u32,
    extra_ammount: u32,
    killshot: bool,
}

impl Unit {
    pub(crate) fn new(
        pos_world_x: usize,
        pos_world_y: usize,
        health: u32,
        strength: u32,
        energy: u32,
        vision_range: usize,
    ) -> Self {
        Self {
            pos_world_x,
            pos_world_y,
            vision: UnitVision::new(vision_range),
            health,
            strength,
            energy,
        }
    }

    pub(crate) fn movement(&mut self, x: usize, y: usize) {
        self.pos_world_x = x;
        self.pos_world_y = y;
        self.vision.update_vision();
    }

    pub(crate) fn attack(&mut self, enemy: &mut Unit) -> DamageStat {
        // TODO Improve with range check etc (return Result with possible errors)
        let dmg = enemy.take_damage(self.strength);
        self.energy -= self.strength;
        return dmg;
    }

    pub(crate) fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub(crate) fn is_dead(&self) -> bool {
        !self.is_alive()
    }

    fn take_damage(&mut self, damages: u32) -> DamageStat {
        let mut damage_applied: DamageStat = DamageStat {
            effective_damage: 0,
            extra_ammount: 0,
            killshot: false,
        };

        if damages >= self.health {
            damage_applied.effective_damage = self.health;
            damage_applied.extra_ammount = damages - self.health;
            damage_applied.killshot = true;
        } else {
            damage_applied.effective_damage = damages;
            damage_applied.extra_ammount = 0;
            damage_applied.killshot = false;
        }

        self.health -= damage_applied.effective_damage;
        return damage_applied;
    }
}

impl UnitVision {
    fn new(vision_range: usize) -> Self {
        // The vision grid is a square that includes the unit position + the range in all directions (width and height)
        let row_size = (vision_range * 2) + 1;
        let column_size = row_size;
        let vision_grid = Vec::with_capacity(row_size * column_size);

        Self {
            vision_range,
            vision_width: row_size,
            vision_height: column_size,
            vision_grid: vision_grid,
        }
    }

    fn update_vision(&mut self) {
        todo!("Not Implemented");
    }
}
