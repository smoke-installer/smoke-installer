mod games;

use libl::{curl, zip};
use std::fs::{copy, remove_file};

pub fn install_game(game: &str) {
    if game == "Hello Smoke" {
        games::hello_smoke();
    } else {
        // println!("Game not found");
    }
}
