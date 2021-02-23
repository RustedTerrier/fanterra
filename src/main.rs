mod setup;
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
        setup::create_world();
    }
    if choice[0..choice.len() - 1] == String::from("2") {
        //If you want to play an existing game, fuck you.
    }
    if choice[0..choice.len() - 1] == String::from("3") {
        //If you want to quit, quit.
        println!("Quiting...");
    }
}
