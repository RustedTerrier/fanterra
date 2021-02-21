use rand::Rng;
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
        create_world();
    }
}

fn create_world() {
    let seed = create_seed();
    println!("{}", seed);
}

fn create_seed() -> String {
    println!("Would you like to enter a seed? [By default one will be generated for you.]");

    let mut answer = String::new();
    let mut seed2 = String::new();
    let seed: &str;
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");
    if answer.to_ascii_uppercase() == "Y\n".to_string() {
        println!("Enter your seed: [Keep in mind that valid seeds are only made of numbers and have a length of 12]");
        io::stdin()
            .read_line(&mut seed2)
            .expect("Failed to read line.");
        seed = &seed2[0..seed2.len() - 1];
    } else {
        while seed2.len() < 12 {
            seed2 = format!("{}{}", seed2, rand::thread_rng().gen_range(0, 10));
        }
        seed = &seed2[0..];
    }

    seed.to_string()
}
