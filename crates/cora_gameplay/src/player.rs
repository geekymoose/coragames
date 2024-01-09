use crate::action::Action;

pub struct Player {
}

enum PlayerRequestStatus {
    Sending,
    Pending,
    Responded,
}

struct PlayerActionRequest {
    status: PlayerRequestStatus,
    action: Option<Action>,
}