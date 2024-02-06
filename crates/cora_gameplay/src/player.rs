use crate::unit::Unit;

/// A Player is an entity which controls one unit and decides its actions at each turn.
/// You can view it as the physical player in a boardgame for instance.
#[derive(Debug)]
pub struct Player {
    /// The unique player ID to uniquely designate this player.
    id: u32,

    /// The player's name.
    name: String,

    /// The unit currently controlled by the player.
    /// One unit should be controlled by exactly one player.
    unit: Unit,
}

impl Player {
    pub(crate) fn new(id: u32, name: String, unit: Unit) -> Self {
        Self { id, name, unit }
    }

    pub fn id(&self) -> &u32 {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn unit(&self) -> &Unit {
        return &self.unit;
    }
}
