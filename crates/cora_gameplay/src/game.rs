use crate::{player::Player, terrain::Grid};

struct Game {
    current_turn: usize,
    grid: Grid,
    players: Vec<Player>,
}
