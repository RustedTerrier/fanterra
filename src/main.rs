mod game;
mod setup;
use std::env;
use std::io;

fn main() {
    start_screen();
}

fn start_screen() {
    println!("AMMMMMMMMMMA     AMMA      AMMMA     AMA  AMMMMMMMMMMMMMA  AMMMMMMMMMMA  AMMMMMA.     AMMMMMA.         AMMA\nMMMMMMMMMMMV    AMVVMA     MMMMMA    MMM  VMMMMMMMMMMMMMV  MMMMMMMMMMMV  MMMMMMMMA    MMMMMMMMA       AMVVMA\nMMM            AMV  VMA    MMM VMA   MMM        MMM        MMM           MMM`   `MA   MMM`   `MA     AMV  VMA\nMMM           AMV    VMA   MMM  VMA  MMM        MMM        MMM           MMM     ;MA  MMM     ;MA   AMV    VMA\nMMMMMMMMA    AMV      VMA  MMM   VMA MMM        MMM        MMMMMMMMMA    MMM.   .AMV  MMM.   .AMV  AMV      VMA\nMMMMMMMMV    MMMMMMMMMMMM  MMM    VMAMMM        MMM        MMMMMMMMMV    MMMMMMMMMV   MMMMMMMMMV   MMMMMMMMMMMM\nMMM          MMMMMMMMMMMM  MMM     VMMMM        MMM        MMM           MMMMMMMMA    MMMMMMMMA    MMMMMMMMMMMM\nMMM          MMM      MMM  MMM      VMMM        MMM        MMMMMMMMMMMA  MMM    VMA   MMM    VMA   MMM      MMM\nVMV          VMV      VMV  VMV       VMV        VMV        VMMMMMMMMMMV  VMV     VMA  VMV     VMA  VMV      VMV\n");
    println!("Do you want to do? 1) Create a new game, 2) Play an existing game, or 3) Quit?");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    if choice[0..choice.len() - 1] == String::from("1") {
        //If you want to create a new world, create a new world.
        let hme = env::var("HOME").unwrap();
        setup::create_world(hme);
        play_game();
    }
    if choice[0..choice.len() - 1] == String::from("2") {
        play_game();
    }
    if choice[0..choice.len() - 1] == String::from("3") {
        //If you want to quit, quit.
        println!("Quiting...");
    }
}

fn play_game() {
    let v = setup::read_worlds(env::var("HOME").unwrap());
    let mut v_s: String = v.into_iter().collect();
    v_s = v_s[0..v_s.len() - 1].to_string();
    println!(
        "Choose from each world, with a corresponding number: \n\r{}",
        &v_s
    );
    v_s = setup::read_worlds(env::var("HOME").unwrap())
        .into_iter()
        .collect();
    let mut world = String::new();
    io::stdin()
        .read_line(&mut world)
        .expect("Something went wrong reading your input.");
    let worldnum = world.replace("\n", "").parse::<u32>().unwrap() - 1;
    let worldv: Vec<&str> = v_s.split("\n").collect();
    world = worldv[worldnum as usize].to_string();
    world = world[2..].to_string();
    let game = setup::setup_game(world, env::var("HOME").unwrap());
    game::start_game(game.seed, game.pa1, game.pa2, game.pa3);
}
