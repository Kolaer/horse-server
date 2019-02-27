extern crate serde;

mod types;
mod gamesate;

use gamesate::GameState;

fn main() {
    let game_sate = GameState::default();

    println!("{:#?}", game_sate);
}
