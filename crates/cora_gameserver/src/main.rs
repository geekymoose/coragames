use cora_gameplay::{action::Action, game::Game, terrain::Direction};
use std::{thread, time::Duration};

const TURN_DURECTION_SEC: u64 = 1;

fn main() {
    println!("--- Cora GameServer starts ---");

    let config: cora_gameplay::terrain::Config = cora_gameplay::terrain::Config {
        width: 100,
        height: 100,
    };

    let mut game = Game::new(config);

    add_player(1, String::from("player1"), &mut game);
    add_player(2, String::from("player2"), &mut game);

    loop {
        println!("Playing one turn...");
        println!("--- DEBUG (dumping game data): ---\n{:?}", game);

        game.request_turn_action();

        game.register_player_response(1, Action::Move(Direction::Up));
        game.register_player_response(2, Action::Move(Direction::Up));

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
