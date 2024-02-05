use crate::config::*;

/// Represents one unit currently ingame.
/// A unit is a something that can interact in the grid, move, attack.
/// It is meant to be controlled by a player or an AI.
#[derive(Debug)]
pub(crate) struct Unit {
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
    pub(crate) fn new(health: u32, strength: u32, vision_range: usize) -> Self {
        Self {
            health,
            strength,
            vision_range,
        }
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

impl Default for Unit {
    fn default() -> Self {
        Self {
            health: DEFAULT_UNIT_HEALTH,
            strength: DEFAULT_UNIT_STRENGTH,
            vision_range: DEFAULT_UNIT_VISION_RANGE,
        }
    }
}
