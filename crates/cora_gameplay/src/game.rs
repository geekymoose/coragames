use crate::{terrain::Grid, player::Player};

struct Game {
    current_turn: usize,
    grid: Grid,
    players: Vec<Player>,
}
