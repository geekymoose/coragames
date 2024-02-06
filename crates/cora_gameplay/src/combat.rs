#[derive(Debug)]
pub struct Health {
    current_health: u32,
    max_health: u32,
}

#[derive(Debug)]
pub struct Weapon {
    power: u32,
}

#[derive(Debug)]
pub struct DamageStat {
    effective_damage: u32,
    extra_ammount: u32,
    is_killshot: bool,
}

impl Health {
    pub(crate) fn new(health: u32) -> Self {
        Self {
            current_health: health,
            max_health: health,
        }
    }

    fn take_damage(&mut self, damages: u32) -> DamageStat {
        let mut damage_applied: DamageStat = DamageStat {
            effective_damage: 0,
            extra_ammount: 0,
            is_killshot: false,
        };

        if damages >= self.current_health {
            damage_applied.effective_damage = self.current_health;
            damage_applied.extra_ammount = damages - self.current_health;
            damage_applied.is_killshot = true;
        } else {
            damage_applied.effective_damage = damages;
            damage_applied.extra_ammount = 0;
            damage_applied.is_killshot = false;
        }

        self.current_health -= damage_applied.effective_damage;
        return damage_applied;
    }
}

impl Weapon {
    pub fn new(power: u32) -> Self {
        Self { power }
    }

    pub fn attack(&mut self, enemy: &mut Health) -> DamageStat {
        // TODO Improve with range check etc (return Result with possible errors)
        let dmg = enemy.take_damage(self.power);
        return dmg;
    }
}
