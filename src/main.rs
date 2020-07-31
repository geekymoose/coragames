struct Player {
    x: u32,
    y: u32,
}

struct Tilemap {
    map: [u32; 200], // TMP hardcoded values
}

struct game {
    tilemap: Tilemap,
    players: [Player; 5], // TMP hardcoded values
}

fn main() {
    println!("--- Cora game server starts ---");

    let turn_duration_seconds = 1.0;

    loop {
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
