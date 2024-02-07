use std::{thread, time::Duration};

use cora_gameplay::{action::Action, direction::Direction, game::Game};

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
        //println!("--- DEBUG (dumping game data): ---\n{:?}", game);

        let requests = game.request_turn_action();

        for req in requests {
            let id = req.0;
            let action = Action::Move(Direction::Up);

            match game.register_turn_action_response(id, action) {
                Ok(_) => println!("Successfully registered a response for unit ID: {}", id),
                Err(msg) => println!(
                    "Error: failed to register the response for unit ID: {} with error: {}",
                    id, msg
                ),
            }
        }

        thread::sleep(Duration::from_millis(game.turn_duraction_in_ms() as u64));
        game.apply_turn();
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
