use std::io;

pub fn start_game(seed: u64, map1: Vec<u8>, map2: Vec<u8>, map3: Vec<u8>) -> GameData {
    // Prompt user for the class
    let mut choice = String::new();
    println!("What class do you want? 1) Warrior, 2) Wizard, 3) Bard, 4) Rogue");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let class: u8;
    loop {
        match &choice[..] {
            "1\n" => {
                class = 1;
                break;
            }
            "2\n" => {
                class = 2;
                break;
            }
            "3\n" => {
                class = 3;
                break;
            }
            "4\n" => {
                class = 4;
                break;
            }
            _ => {
                // Check if you insert the right thing this time.
                println!("What class do you want? 1) Warrior, 2) Wizard, 3) Bard, 4) Rogue (This has to be a number from 1-4)");
                // Reset choice because for some reason it just gets added otherwise
                choice = "".to_string();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read line");
            }
        }
    }
    let mut player = GameData {
        class: class,
        name: "".to_string(),
        seed: seed,
        p1: map1,
        p2: map2,
        p3: map3,
        path: 0,
        area: 0,
    };

    // Get name for some reason
    println!("What is your name?");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    player.name = choice;

    // Print the backstory and setup.
    println!("\n\
        A group of goblins invade the town.\n\
        You run to hide and they don't notice you. As you hide, you hear sounds of destruction.\n\
        You leave your hiding spot and see an old man crawl out from the rubble, he seems weary.\n\
        He approaches you. The old man talks to you: \"There is a castle down that road, I've seen it with my own eyes, \n\
        full of goblins, catapults, and other siege weapons. You must go there, kill the general and burn the catapults\n\
        if you wish to save the kingdom.\"");
    let weapon = match player.class {
        1 => "Wooden Club",
        2 => "Earth Staff",
        3 => "Spruce Lute",
        4 => "Iron Dagger",
        _ => "punch to the face",
    };

    // Give weapon
    println!(
        "\nThe old man gives you a {}, sending you on your quest.",
        weapon
    );

    // Tell location of one town
    println!("\n\
    Before you leave, the old man talks to you again: \"There is a town on the left path after the fork, it's close \n\
        by and they will help you if you're hurt.\"\n\
        Then he leaves, and you embark on your journey.\n");

    // Start the journey
    println!("You walk down the given path and the journey is uneventful up until a fork in the road. You remember the old\n\
        man saying there was a town somewhere to the left.\n\
        Do you wish to go 1) To the left, 2) Straight ahead, or 3) To the right?");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    player = fork_in_the_road(choice, player);
    player
}

pub fn resume_game() {}

fn fork_in_the_road(mut choice: String, mut world: GameData) -> GameData {
    // Thing is here for the match statement so I don't have to set stuff to stuff it won't be.
    let _thing;

    let choice_as_u8: u8;
    loop {
        match &choice[..] {
            "1\n" => {
                choice_as_u8 = 1;
                break;
            }
            "2\n" => {
                choice_as_u8 = 2;
                break;
            }
            "3\n" => {
                choice_as_u8 = 3;
                break;
            }
            _ => {
                // Check if you insert the right thing this time.
                println!("Do you wish to go 1) To the left, 2) Straight ahead, or 3) To the right? (This has to be a number from 1-3)");
                // Reset choice because for some reason it just gets added to otherwise
                choice = "".to_string();
                io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read line");
            }
        }
    }

    // Set the current path and area.
    match choice_as_u8 {
        1 => {
            _thing = {
                world.area = navigate_path(&world.p1, 1);
                world.path = 1;
            }
        }
        2 => {
            _thing = {
                world.area = navigate_path(&world.p2, 1);
                world.path = 2;
            }
        }
        3 => {
            _thing = {
                world.area = navigate_path(&world.p3, 1);
                world.path = 3;
            }
        }
        _ => _thing = (),
    };
    // Return updated world with the current area and path.
    world
}

pub struct GameData {
    class: u8,
    name: String,
    seed: u64,
    p1: Vec<u8>,
    p2: Vec<u8>,
    p3: Vec<u8>,
    path: u8,
    area: u8,
}

fn navigate_map(world: GameData) {
    let mut area = 1;
    area = navigate_path(&world.p1, area);
    loop {
        area = navigate_path(&world.p1, area);
    }
}

fn navigate_path(path: &Vec<u8>, area: u8) -> u8 {
    // Set the message for entering a new area
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

fn encounter_enemy(area: u8) {}
