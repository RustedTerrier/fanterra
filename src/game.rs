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
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    println!("\nBefore you leave, the old man talks to you again: \"There is a town on the left path after the fork, it's close \nby and they will help you if you're hurt.\"\nThen he leaves, and you embark on your journey.\n");
    println!("You walk down the given path and the journey is uneventful up until a fork in the road. You remember the old\nman saying there was a town somewhere to the left.\nDo you wish to go 1) To the left, 2) To the straight, or 3) To the right?");
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
    let _thing;
    match choice.replace("\n", "").parse::<u8>().unwrap() {
        1 => _thing = navigate_path(&world.p1, 1),
        2 => _thing = navigate_path(&world.p2, 1),
        3 => _thing = navigate_path(&world.p3, 1),
        _ => _thing = 0,
    };
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

fn navigate_map(world: World) {
    let mut area = 1;
    area = navigate_path(&world.p1, area);
    loop {
        area = navigate_path(&world.p1, area);
    }
}

fn navigate_path(path: &Vec<u8>, area: u8) -> u8 {
    let biome = match path[(area - 1) as usize] {
        0 => "You enter a beautiful forest which eerily fades into darkness as the shadows of the trees cover the ground. It feels strangely calm yet threatening, what do you do?",
        1 => "You enter a town, to the left you see a sort of hospital, up ahead is a tavern, and to the right are houses and a shop; the people feel are welcoming, what do you do?",
        2 => "You walk into a plain of hills, you feel uneasy; you never know whats around the next hill but the green of the plants feels calming and it confuses you; what do you do?",
        3 => "You enter an outpost and watch from afar as enemies enter and leave a large stone building; they are armed to the teeth. What do you do?",
        4 => "You enter a barren wasteland, only sand covers the ground as far as you can see and it's hot to the touch, what do you do?",
        5 => "You enter a swamp and water goes up to your waist. The murky water hides any predators from view and you feel scared. What do you do?",
        6 => "You enter a beautiful forest which eerily fades into darkness as the shadows of the trees cover the ground. It feels strangely calm yet threatening, what do you do?",
        7 => "You find yourself at the castle walls, what do you do?",
        _ => "",
    };

    println!("{}", biome);
    area
}
