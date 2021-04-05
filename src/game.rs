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
            | "1\n" => {
                class = 1;
                break;
            },
            | "2\n" => {
                class = 2;
                break;
            },
            | "3\n" => {
                class = 3;
                break;
            },
            | "4\n" => {
                class = 4;
                break;
            },
            | _ => {
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
        class,
        name: "".to_string(),
        seed,
        p1: map1,
        p2: map2,
        p3: map3,
        path: 0,
        area: 0,
        game_state: true
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
        He approaches you. The old man talks to you: \"There is a castle down that road, I've seen it with my own\n\
        eyes, full of goblins, catapults, and other siege weapons. You must go there, kill the general and burn the\n\
        catapults if you wish to save the kingdom.\"");
    let weapon = match player.class {
        | 1 => "Wooden Club",
        | 2 => "Earth Staff",
        | 3 => "Spruce Lute",
        | 4 => "Iron Dagger",
        | _ => "punch to the face"
    };

    // Give weapon
    println!(
        "\nThe old man gives you a {}, sending you on your quest.",
        weapon
    );

    // Tell location of one town
    println!("\n\
    Before you leave, the old man talks to you again: \"There is a town on the left path after the fork, it's \n\
    close by and they will help you if you're hurt.\"\n\
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

pub fn resume_game() {
}

fn fork_in_the_road(mut choice: String, mut world: GameData) -> GameData {
    // Thing is here for the match statement so I don't have to set stuff to stuff it won't be.
    let _thing;

    let choice_as_u8: u8;
    loop {
        match &choice[..] {
            | "1\n" => {
                choice_as_u8 = 1;
                break;
            },
            | "2\n" => {
                choice_as_u8 = 2;
                break;
            },
            | "3\n" => {
                choice_as_u8 = 3;
                break;
            },
            | _ => {
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
        | 1 => {
            _thing = {
                world.area = navigate_path(&world.p1, 1);
                world.path = 1;
            }
        },
        | 2 => {
            _thing = {
                world.area = navigate_path(&world.p2, 1);
                world.path = 2;
            }
        },
        | 3 => {
            _thing = {
                world.area = navigate_path(&world.p3, 1);
                world.path = 3;
            }
        },
        | _ => _thing = ()
    };
    // Return updated world with the current area and path.
    world
}

pub struct GameData {
    pub class:      u8,
    pub name:       String,
    pub seed:       u64,
    pub p1:         Vec<u8>,
    pub p2:         Vec<u8>,
    pub p3:         Vec<u8>,
    pub path:       u8,
    pub area:       u8,
    pub game_state: bool
}

pub fn navigate_path(path: &Vec<u8>, mut area: u8) -> u8 {
    // Set the message for entering a new area
    let biome = match path[(area - 1) as usize] {
        // 0 is a forest, 1 is a town, 2 is hills, 3 is an outpost, 4 is a desert, 5 is a swamp,
        //   and 6 is a hideout.
        0 => format!("You enter a beautiful forest which eerily fades into darkness as the shadows of the trees cover the ground. It\n\rfeels strangely calm yet threatening, what do you do?"),
        1 => format!("You enter a town, to the left you see a sort of hospital, up ahead is a tavern, and to the right are houses and a\n\rshop; the people feel welcoming, what do you do?"),
        2 => format!("You walk into a plain of hills and you begin to feel uneasy; you never know whats around the next hill but the\n\rgreen of the plants feels calming and it confuses you; what do you do?"),
        3 => format!("You enter an outpost and watch from afar as enemies enter and leave a large stone building; they are armed to the\n\rteeth. What do you do?"),
        4 => format!("You enter a barren wasteland, only sand covers the ground as far as you can see and it's hot to the touch, what\n\rdo you do?"),
        5 => format!("You enter a swamp and water goes up to your waist. The murky water hides any predators from view and you feel \n\rscared. What do you do?"),
        6 => format!("You enter a beautiful forest which eerily fades into darkness as the shadows of the trees cover the ground. It \n\rfeels strangely calm yet threatening, what do you do?"),
        7 => format!("You find yourself at the castle walls, what do you do?"),
        _ => format!(""),
    };

    println!("{}", biome);
    // Just make sure they use the right action.
    loop {
        let action = actions(0);
        match &action[..] {
            | "1\n" => {
                look_around(path[(area - 1) as usize]);
            },
            | "2\n" => {
                break;
            },
            | "3\n" => {
                if area != 1 {
                    println!("You turn back and leave.");
                    area -= 2;
                    break;
                }
                else {
                    println!("You musn't turn back now.");
                }
            },
            | _ => {}
        }
    }
    area
}

fn actions(situation: u8) -> String {
    match situation {
        | 0 => println!("1) Look around, 2) Cross the landscape, 3) Leave"),
        | _ => {}
    }
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    loop {
        match &response[..] {
            | "1\n" => {
                break;
            },
            | "2\n" => {
                break;
            },
            | "3\n" => {
                break;
            },
            | _ => {
                // Check if you insert the right thing this time.
                match situation {
                    | 0 => println!("1) Look around, 2) Cross the landscape, 3) Leave"),
                    | _ => {}
                }
                // Reset choice because for some reason it just gets added to otherwise
                response = "".to_string();
                io::stdin()
                    .read_line(&mut response)
                    .expect("Failed to read line");
            }
        }
    }
    response
}

fn look_around(biome: u8) {
    match biome {
        // 0 is a forest, 1 is a town, 2 is hills, 3 is an outpost, 4 is a desert, 5 is a swamp,
        //   and 6 is a hideout.
        | 0 => {
            print!("You look around, the trees are far taller than you and the father the forest goes, the less light hits the\n\r\
                      ground. You hear sounds of movement but you can't identify anything. There seems to be foot prints in the mud\n\r\
                      and you see arrows sticking out of the ground just up ahead.");
        },
        | 1 => {
            print!("The townsfolk seem friendly. The pavement on the ground forms a diamond pattern and it looks like it's\n\r\
                      made out of stone. The buildings are made out of wood or stone painted white with limestone.");
        },
        | 2 => {
            print!("The hills are very steep, almost mountainous and you can see birds flying overhead. There is no where to hide\n\r\
                      should any enemies find you atop one of the hills. You see reminents of a camp inbetween some of the hills.");
        },
        | 3 => {
            print!("You can see the base is made out of placed rocks with some sort of limestone mix holding them togethor. The\n\r\
                      middle of the outpost is made from wood, it seems to be weakened and cheap. At the top you see archers in\n\r\
                      steal chainmail and with wooden bows. You see a body hung from the top with a sign but you can't make out the\n\r\
                      words.");
        },
        | 4 => {
            print!("As you glance among the landscape you see only yellows and faint greens. There are small shrubs in between\n\r\
                      the sand. You can easily be spotted from the sand dunes and you begin to feel hot; no water is in sight. On\n\r\
                      one of the dunes you can see the sand painted red by a body; you feel uneasy.");
        },
        | 5 => {
            print!("The water is murky and the ground is littered with mud. The trees here cast dark shadows as they lay in the\n\r\
                      murky waters. You see ripples in the water, and hear the sounds of snakes and gators. There are bloody\n\r\
                      remains of some sort of men.")
        },
        | 6 => {
            print!("You look around, the trees are far taller than you and the father the forest goes, the less light hits the\n\r\
                      ground. You hear sounds of movement but you can't identify anything. There seems to be foot prints in the mud\n\r\
                      and you see arrows sticking out of the ground just up ahead.");
        },
        | _ => {
            panic!("Uhh, this area doesn't exist or it doesn't support looking around.");
        }
    }
    println!("\nWhat do you do?");
}

fn encounter_enemy(area: u8, biome: &str, seed: u64) {
    // Plan for now: the farther along the path the harder the enemies and better the loot. Seed
    // determines the drop and there are 4 drops on each possible area and after 4 battles you have
    // them all.
    //
    // Forest Enemies:
    // 1: Ratt, Wolf
    // 2: Wolf, Owl
    // 3: Warg, Eagle
    // 4: Bear, Giant Owl
    // 5: Bandit, Archer
    // 6: Warg-Rider, Soldier
    // 7: Soldiers, Doleney(Tree-Person)
    // 8+: Spearmen, Klorx
    //
    // Hill Enemies:
    // 1: Rat, Weeds
    // 2: Bobcat, Sparrow,
    // 3: Zombie, Owl,
    // 4: Weeping Willow, Goblin
    // 5: Adventurer, Undead
    // 6: Undead, Troll
    // 7: Undead, Ogre,
    // 8+: Ogre, Undead
    //
    // Outpost Enemies:
    // 1: Scout, Mouse
    // 2: Soldier, Crossbowman
    // 3: Soldier, Swordsman
    // 4: Spearmen, Archer
    // 5: Yentu(Tanky guy), Soldiers
    // 6: Kralkon(Very Fast, moderate tank), Yentu
    // 7: Kralkons, Yentus
    // 8+: Cafras(Extremely high attack), Srew(Kralkon and Yentu togethor)
    //
    // Desert Enemies:
    // 1: Scorpions, Fox
    // 2: Begino(Giant Beetle), Scout
    // 3: Camel, Eraldun(Giant cat type thing)
    // 4: Eraldun, Begino
    // 5: Soldier, Eraldun
    // 6: Est(Giant Worm), Soldier
    // 7: Est, Adventurer
    // 8+: Sand-Dragon, Sand-Mage
    //
    // Swamp Enemies:
    // 1: Piranha, Snapping Turtle
    // 2: Barracuda, Piranha
    // 3: Snake, Tyosil(Giant Frog)
    // 4: Tyosil, Eel
    // 5: Octopus, Soldier
    // 6: Crocodile, White-snake
    // 7: Yentu, Crocodile
    // 8+: Crocodile, Yentu
    //
    // Hideout Enemies:
    // 1: Bandit, Scout
    // 2: Soldier, Archer
    // 3: Warg, Soldier
    // 4: Soldiers, Spearmen
    // 5: Soldiers, Bear
    // 6: Adventurer, Executer
    // 7: Executer, Yentu
    // 8+: Executer, Rogue
}
