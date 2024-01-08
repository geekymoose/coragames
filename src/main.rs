// WIP This is a totally WIP state
// Still used for learning purpose (trying out Rust)

use std::{thread, time::Duration};

// TODO Update this constants with a dyn config
const PLAYERS_COUNT: usize = 5;
const TIMEMAP_SIZE: usize = 200;
const TURN_DURECTION_SEC: u64 = 1;

struct GameTilemapGrid {
    map: [GameTilemapCell; TIMEMAP_SIZE],
}

struct GameTilemapCell {
    unit: Option<Unit>,
    terrain: EnvironmentType,
}

struct Unit {
    id: u32,
}

impl Unit {
    // TODO
    fn create_view() {}
}

enum EnvironmentType {
    Movable,
    Impassable,
}

enum Action {
    Move(Direction),
    Attack(Direction),
    Interact(Direction),
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum PlayerRequestStatus {
    Sending,
    Pending,
    Responded,
    Idle,
}

struct PlayerActionRequest {
    vision: [u32; 5],
    status: PlayerRequestStatus,
    start_turn: usize,
    action: Option<Action>,
}

struct Game<'a> {
    current_turn: usize,
    tilemap: GameTilemapGrid,
    players: [Player<'a>; PLAYERS_COUNT],
    players_actions: [Option<PlayerActionRequest>; PLAYERS_COUNT],
}

struct Player<'a> {
    unit: &'a Unit,
}

impl<'a> Game<'a> {
    fn play_one_turn(&mut self) {
        self.current_turn += 1;
        // TODO WIP
    }

    fn new() -> Game<'a> {
        todo!()
    }
}

impl<'a> Default for Game<'a> {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    println!("--- Cora game server starts ---");

    let mut game: Game = Default::default();

    loop {
        println!("Playing one turn...");
        game.play_one_turn();
        thread::sleep(Duration::from_secs(TURN_DURECTION_SEC));

        // Message Connect / Disconnect player
        //
        // Create each player "view" info
        // Ask each player for action (send "view" info)
        // Wait until "turn duration" (sleep thread for)
        // Skipp player if not received on time
        // Apply player actions (update game)
    }

    println!("--- Cora game server stops ---");
}
