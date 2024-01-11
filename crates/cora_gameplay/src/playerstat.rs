use crate::action::Action;

pub(crate) struct PlayerStat {
    /// Player that is related to these stats.
    pub player_id: u32,

    /// Turn when the player joined the game.
    /// This is sually 0 if joined at start.
    pub turn_joined: u32,

    /// Number of turns this player has been in-game.
    /// Regardless of the actual action done.
    pub turns_spent_ingame: u32,

    /// Number of turns when this player provided an action to play.
    pub turn_action_applied: u32,

    /// Number of turns when this player failed to provide an action to play.
    pub turn_action_missed: u32,
}

impl PlayerStat {
    pub(crate) fn new(player_id: u32, turn_joined: u32) -> Self {
        Self {
            player_id,
            turn_joined,
            turns_spent_ingame: 0,
            turn_action_applied: 0,
            turn_action_missed: 0,
        }
    }

    pub(crate) fn update_played_one_turn(&mut self, action: &Option<Action>) {
        self.turns_spent_ingame += 1;

        match action {
            Some(_) => self.turn_action_applied += 1,
            None => self.turn_action_missed += 1,
        }
    }
}
