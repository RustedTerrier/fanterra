mod setup;

use rand::Rng;
use setup::*;
use std::io;

fn main() {
    start_screen();
}

fn start_screen() {
    println!("Do you want to do? 1) Create a new game, 2) Play an existing game, or 3) Quit?");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    if choice[0..choice.len() - 1] == String::from("1") {
        setup::create_world();
    }
}
