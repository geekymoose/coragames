use bevy::prelude::*;

use crate::plugins::gameplay::GameplayPlugin;

mod components;
mod entities;
mod plugins;
mod resources;
mod systems;

fn main() {
    println!("--- Cora GameServer starts ---");

    App::new().add_plugins(GameplayPlugin).run();

    println!("--- Cora GameServer stops ---");
}
