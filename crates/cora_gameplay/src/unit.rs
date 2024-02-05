// TODO This is to improve (for now, just easy WIP)
const DEFAULT_UNIT_VISION_RANGE: usize = 5;
const DEFAULT_UNIT_STRENGTH: u32 = 10;
const DEFAULT_UNIT_HEALTH: u32 = 100;

/// Represents one unit currently ingame.
/// A unit is a something that can interact in the grid, move, attack.
/// It is meant to be controlled by a player or an AI.
#[derive(Debug)]
pub(crate) struct Unit {
    // TODO To update with methods
    pub pos_world_x: usize,
    pub pos_world_y: usize,
    health: u32,
    strength: u32,
    vision_range: usize,
}

#[derive(Debug)]
pub(crate) struct DamageStat {
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
        vision_range: usize,
    ) -> Self {
        Self {
            pos_world_x,
            pos_world_y,
            health,
            strength,
            vision_range,
        }
    }

    pub(crate) fn new_default(pos_world_x: usize, pos_world_y: usize) -> Self {
        Self {
            pos_world_x,
            pos_world_y,
            health: DEFAULT_UNIT_HEALTH,
            strength: DEFAULT_UNIT_STRENGTH,
            vision_range: DEFAULT_UNIT_VISION_RANGE,
        }
    }

    pub(crate) fn movement(&mut self, x: usize, y: usize) {
        self.pos_world_x = x;
        self.pos_world_y = y;
    }

    pub(crate) fn attack(&mut self, enemy: &mut Unit) -> DamageStat {
        // TODO Improve with range check etc (return Result with possible errors)
        let dmg = enemy.take_damage(self.strength);
        return dmg;
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
