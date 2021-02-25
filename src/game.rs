use std::io;

pub fn start_game(seed: u64) ->  character {
    let mut choice = String::new();
    println!("What class do you want? 1) Warrior, 2)Wizard, 3)Bard, 4)Rogue");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let mut player = new character {
        class: choice as u32,
        name: "".to_string(),
    }
    println!("What is your name?");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    player.name = choice;
}

pub fn resume_game() {

}

pub struct character {
    class: u8,
    name: String,
}
