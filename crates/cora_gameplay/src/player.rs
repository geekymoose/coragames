use crate::action::Action;
use crate::playerstat::PlayerStat;
use crate::terrain::Grid;
use crate::unit::Unit;

/// A Player is an entity which controls one unit and decides its actions at each turn.
/// You can view it as the physical player in a boardgame for instance.
pub(crate) struct Player<'a> {
    /// The unique player ID to uniquely designate this player.
    id: u32,

    /// The player's name.
    name: String,

    /// The unit currently controlled by the player.
    /// One unit should be controlled by exactly one player.
    unit: &'a Unit,

    /// Each turn, the player has to compute an action.
    /// This is the current request ongoing and it's possible response when computed.
    /// If None, it means the player is currently not computing anything (e.g., start of the turn).
    turn_action: PlayerTurnStatus,

    /// Player stats for the current game.
    stats: PlayerStat,
}

enum PlayerTurnStatus {
    /// The player is not doing anything.
    /// Usually, this is at the beginning of the turn.
    Idle,

    /// The player is computing an action.
    /// This holds the data used for the request.
    Computing(PlayerTurnRequest),

    /// The player responded with an action to do.
    /// This holds the response data (the action).
    Responded(PlayerTurnResponse),
}

struct PlayerTurnRequest {
    turn_start: u32,
    vision: Grid,
}

struct PlayerTurnResponse {
    turn_start: u32,
    turn_end: u32,
    action: Action,
}

impl<'a> Player<'a> {
    /// Creates a new player for the provided unit.
    pub(crate) fn new(id: u32, name: String, unit: &'a Unit, current_turn: u32) -> Self {
        Self {
            id,
            name,
            unit,
            turn_action: PlayerTurnStatus::Idle,
            stats: PlayerStat::new(id, current_turn),
        }
    }

    pub(crate) fn id(&self) -> &u32 {
        &self.id
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn status(&self) -> &PlayerTurnStatus {
        &self.turn_action
    }

    pub(crate) fn apply_turn_action(&mut self) {
        match &self.turn_action {
            PlayerTurnStatus::Idle => {
                // Nothing to do (this method may have been called to early)
            }
            PlayerTurnStatus::Computing(request) => {
                // The player did not responded yet. Too bad.
            }
            PlayerTurnStatus::Responded(response) => {
                match &response.action {
                    // TODO WIP
                    Action::Move(Direction) => todo!(),
                    Action::Attack(Direction) => todo!(),
                    Action::Interact(Direction) => todo!(),
                }
            }
        }
    }
}
