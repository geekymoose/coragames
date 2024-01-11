use crate::action::Action;
use crate::playerstat::PlayerStat;
use crate::unit::Unit;

pub(crate) struct Player<'a> {
    // The unique player ID to uniquely designate this player.
    pub id: u32,

    // The player name
    pub name: String,

    // The unit currently controlled by the player.
    pub unit: &'a Unit,

    // The action choosen by the player (to apply on the current turn).
    pub action: Option<Action>,

    // Player stats for the current game.
    pub stats: PlayerStat,
}

impl<'a> Player<'a> {
    pub(crate) fn new(id: u32, name: String, unit: &'a Unit, current_turn: u32) -> Self {
        Self {
            id,
            name,
            unit,
            action: None,
            stats: PlayerStat::new(id, current_turn),
        }
    }
}
