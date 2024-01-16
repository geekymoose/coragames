use crate::terrain::{Cell, Direction};

/// Represents one unit currently ingame.
/// A unit is a something that can interact in the grid, move, attack.
/// It is meant to be controlled by a player or an AI.
pub(crate) struct Unit {
    health: u32,
    strength: u32,
}

struct DamageStat {
    effective_damage: u32,
    extra_ammount: u32,
    killshot: bool,
}

impl Unit {
    pub(crate) fn movement(&self, direction: Direction) {
        todo!("Not Implemented");
    }

    pub(crate) fn attack(&self, direction: Direction) {
        todo!("Not Implemented");
    }

    pub(crate) fn interact(&self, direction: Direction) {
        todo!("Not Implemented");
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
