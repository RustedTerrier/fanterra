mod game;
mod setup;
use std::env;
use std::io;

fn main() {
    start_screen();
}

fn start_screen() {
    println!("AMMMMMMMMMMA     AMMA      AMMMA     AMA  AMMMMMMMMMMMMMA  AMMMMMMMMMMA  AMMMMMA.     AMMMMMA.         AMMA\n\
              MMMMMMMMMMMV    AMVVMA     MMMMMA    MMM  VMMMMMMMMMMMMMV  MMMMMMMMMMMV  MMMMMMMMA    MMMMMMMMA       AMVVMA\n\
              MMM            AMV  VMA    MMM VMA   MMM        MMM        MMM           MMM`   `MA   MMM`   `MA     AMV  VMA\n\
              MMM           AMV    VMA   MMM  VMA  MMM        MMM        MMM           MMM     ;MA  MMM     ;MA   AMV    VMA\n\
              MMMMMMMMA    AMV      VMA  MMM   VMA MMM        MMM        MMMMMMMMMA    MMM.   .AMV  MMM.   .AMV  AMV      VMA\n\
              MMMMMMMMV    MMMMMMMMMMMM  MMM    VMAMMM        MMM        MMMMMMMMMV    MMMMMMMMMV   MMMMMMMMMV   MMMMMMMMMMMM\n\
              MMM          MMMMMMMMMMMM  MMM     VMMMM        MMM        MMM           MMMMMMMMA    MMMMMMMMA    MMMMMMMMMMMM\n\
              MMM          MMM      MMM  MMM      VMMM        MMM        MMMMMMMMMMMA  MMM    VMA   MMM    VMA   MMM      MMM\n\
              VMV          VMV      VMV  VMV       VMV        VMV        VMMMMMMMMMMV  VMV     VMA  VMV     VMA  VMV      VMV\n\
              ");
    println!("Do you want to do? 1) Create a new game, 2) Play an existing game, or 3) Quit?");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    if choice[0..choice.len() - 1] == String::from("1") {
        // If you want to create a new world, create a new world.
        let hme = env::var("HOME").unwrap();
        setup::create_world(hme);
        play_game();
    }
    if choice[0..choice.len() - 1] == String::from("2") {
        // If you want to play an existing game, do that.
        play_game();
    }
    if choice[0..choice.len() - 1] == String::from("3") {
        // If you want to quit, quit.
        println!("Quiting...");
    }
}

fn play_game() {
    let mut worlds_string: String = setup::read_worlds(env::var("HOME").unwrap())
        .into_iter()
        .collect();
    worlds_string = worlds_string[0..worlds_string.len() - 1].to_string();

    println!(
        "Choose from each world, with a corresponding number: \n\r{}",
        &worlds_string
    );
    worlds_string = setup::read_worlds(env::var("HOME").unwrap())
        .into_iter()
        .collect();

    let mut world = String::new();
    io::stdin()
        .read_line(&mut world)
        .expect("Something went wrong reading your input.");

    let worldnum = world.replace("\n", "").parse::<u32>().unwrap() - 1;
    let worldsplit: Vec<&str> = worlds_string.split("\n").collect();
    world = worldsplit[worldnum as usize].to_string();
    world = world[2..].to_string();

    let game = setup::setup_game(world, env::var("HOME").unwrap());

    // pa1, is supposed to mean path1, I'm just lazy at typing.
    game::start_game(game.seed, game.pa1, game.pa2, game.pa3);
}
