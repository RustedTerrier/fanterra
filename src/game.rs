use std::io;

pub fn start_game(seed: u64, map1: Vec<u8>, map2: Vec<u8>, map3: Vec<u8>) -> Character {
    let mut choice = String::new();
    println!("What class do you want? 1) Warrior, 2) Wizard, 3) Bard, 4) Rogue");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let mut player = Character {
        class: choice.replace("\n", "").parse::<u16>().unwrap(),
        name: "".to_string(),
    };
    println!("What is your name?");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    player.name = choice;
    println!("\nA group of goblins invade the town.\nYou run to hide and they don't notice you. As you hide, you hear sounds of destruction.\nYou leave your hiding spot and see an old man crawl out from the rubble, he seems weary.\nHe approaches you. The old man talks to you: \"There is a castle down that road, I've seen it with my own eyes, \nfull of goblins, catapults, and other siege weapons. You must go there, kill the general and burn the catapults\nif you wish to save the kingdom.\"");
    let weapon = match player.class {
        1 => "Wooden Club",
        2 => "Earth Staff",
        3 => "Spruce Lute",
        4 => "Iron Dagger",
        _ => "punch to the face",
    };
    println!(
        "\nThe old man gives you a {}, sending you on your quest.",
        weapon
    );
    println!("\nBefore you leave, the old man talks to you again: \"There is a town on the left path after the fork, it's close \nby and they will help you if you're hurt.\"\nThen he leaves, and you embark on your journey.\n");
    println!("You walk down the given path and the journey is uneventful up until a fork in the road. You remember the old\nman saying there was a town somewhere to the left.\nDo you wish to go 1) To the left, 2) To the straight, or 3) To the right?");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let map = World {
        seed: seed,
        p1: map1,
        p2: map2,
        p3: map3,
    };
    fork_in_the_road(choice, map);
    player
}

pub fn resume_game() {}

fn fork_in_the_road(choice: String, world: World) {
    match choice.replace("\n", "").parse::<u8>().unwrap() {
        1 => navigate_path(world.p1, 1),
        2 => navigate_path(world.p2, 1),
        3 => navigate_path(world.p3, 1),
        _ => {}
    }
}

pub struct Character {
    class: u16,
    name: String,
}

pub struct World {
    seed: u64,
    p1: Vec<u8>,
    p2: Vec<u8>,
    p3: Vec<u8>,
}

fn navigate_path(path: Vec<u8>, area: u8) {
    println!("{}", path[(area - 1) as usize]);
}
