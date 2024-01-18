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

    add_player(1, String::from("player1"), &mut game);
    add_player(2, String::from("player2"), &mut game);

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

    //println!("--- Cora GameServer stops ---");
}

fn add_player(id: u32, name: String, game: &mut Game) {
    match game.add_player(id, name) {
        Ok(player) => println!(
            "Player successfully added: ID: {} // Name: {}",
            player.id(),
            player.name()
        ),
        Err(msg) => println!(
            "Unable to create player. ID: {}\nError message: {}",
            id, msg
        ),
    }
}
