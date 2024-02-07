use std::{thread, time::Duration};

use cora_gameplay::{action::Action, direction::Direction, game::Game};

const TURN_DURECTION_SEC: u64 = 1;

fn main() {
    println!("--- Cora GameServer starts ---");

    let config: cora_gameplay::config::GridConfig = cora_gameplay::config::GridConfig {
        width: 100,
        height: 100,
    };

    let mut game = Game::new(config);

    add_player(1, String::from("player1"), &mut game);
    add_player(2, String::from("player2"), &mut game);

    loop {
        println!("Playing one turn...");
        println!("--- DEBUG (dumping game data): ---\n{:?}", game);

        /*
        game.request_turn_action();

        game.register_player_response(1, Action::Move(Direction::Up));
        game.register_player_response(2, Action::Move(Direction::Up));

        thread::sleep(Duration::from_secs(TURN_DURECTION_SEC));
        game.apply_turn();
         */
    }

    //println!("--- Cora GameServer stops ---");
}

fn add_player(id: u32, name: String, game: &mut Game) {
    match game.add_unit(id, name) {
        Ok(_) => println!("Player successfully added: ID: {} ", id),
        Err(msg) => println!(
            "Unable to create player. ID: {}\nError message: {}",
            id, msg
        ),
    }
}
