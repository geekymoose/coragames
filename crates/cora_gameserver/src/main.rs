use std::{thread, time::Duration};

const TURN_DURECTION_SEC: u64 = 1;

fn main() {
    println!("--- Cora game server starts ---");

    let config: cora_gameplay::terrain::Config = cora_gameplay::terrain::Config {
        width: 100,
        height: 100,
    };

    let mut game: cora_gameplay::game::Game = cora_gameplay::game::Game::new(&config);

    loop {
        println!("Playing one turn...");
        thread::sleep(Duration::from_secs(TURN_DURECTION_SEC));
        game.apply_turn();

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
