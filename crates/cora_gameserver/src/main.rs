use cora_gameplay::game::Game;
use std::{thread, time::Duration};

const TURN_DURECTION_SEC: u64 = 1;

fn main() {
    println!("--- Cora GameServer starts ---");

    let config: cora_gameplay::terrain::Config = cora_gameplay::terrain::Config {
        width: 100,
        height: 100,
    };

    let mut game: Game = Game::new(config);

    // game.spawn_player("player1");
    // game.spawn_player("player2");

    loop {
        println!("Playing one turn...");
        println!("--- DEBUG (dumping game data): ---\n{:?}", game);

        // let turn_action_requests = game.start_turn();
        // for a in turn_action_requests {
        //      TODO send action
        // }
        // TODO Listen for action during TURN_DURECTION_SEC seconds
        // game.register_player_action_turn
        // game.end_turn()
        thread::sleep(Duration::from_secs(TURN_DURECTION_SEC));
        game.apply_turn();
    }

    println!("--- Cora GameServer stops ---");
}
