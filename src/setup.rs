use std::{fs, io, panic, path::Path};

use rand::Rng;

pub fn create_world(home: String) {
    let seed = create_seed().parse::<u64>().unwrap();
    println!("{}", seed);
    let world_name = create_name(&seed.to_string(), 5);
    println!("{}", world_name);
    serialize_base_stuff(world_name, seed, home).expect("Can't create a world file");
}

pub fn setup_game(path: String, home: String) -> Game {
    let map = create_map(&read_file(&path, &home));
    let game = Game {
        pa1:  map.pa1,
        pa2:  map.pa2,
        pa3:  map.pa3,
        seed: read_file(&path, &home)
    };
    game
}

fn create_name(seed: &String, len: u32) -> String {
    let vowels: [char; 5] = ['A', 'E', 'I', 'O', 'U'];
    // Screw y
    let consonants: [char; 19] = [
        'W', 'R', 'T', 'P', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'Z', 'X', 'C', 'V', 'B', 'N',
        'M'
    ];
    let mut number = seed.parse::<f32>().unwrap();
    number = number * 0.03;
    number = number / 836.1;
    let mut name = add_vowel("".to_string(), &number % 10.0, &vowels);
    if len == 5 {
        number = change_seed(number as u64) as f32;
        name = add_consonant(name, &number % 10.0, &consonants);
        number = change_seed(number as u64) as f32;
        name = add_consonant(name, &number % 10.0, &consonants);
        number = change_seed(number as u64) as f32;
        name = add_vowel(name, &number % 10.0, &vowels);
        number = change_seed(number as u64) as f32;
        name = add_consonant(name, &number % 10.0, &consonants);
    }
    name
}
fn add_vowel(word: String, cnum: f32, vowels: &[char; 5]) -> String {
    let name = format!("{}{}", word, vowels[(&cnum % 5.0) as usize].to_string());
    name
}

fn add_consonant(word: String, cnum: f32, consonants: &[char; 19]) -> String {
    let name = format!(
        "{}{}",
        word,
        consonants[(&cnum % 19.0) as usize].to_string()
    );
    name
}

fn create_seed() -> String {
    println!("Would you like to enter a seed? (By default one will be generated for you.) [y/N]");

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
        seed = &seed2[0 .. seed2.len() - 1];
        if seed.chars().count() != 12 {
            panic!("It has to be 12 digits long...");
        }
    }
    else {
        while seed2.len() < 12 {
            seed2 = format!("{}{}", seed2, rand::thread_rng().gen_range(0, 10));
        }
        seed = &seed2[0 ..];
    }

    seed.to_string()
}

pub fn create_map(seed: &u64) -> Map {
    let mut l: u64 = change_seed(*seed);
    l = change_seed(l);
    l = change_seed(l);
    l = change_seed(l);
    let map_style: (u8, u8, u8);
    match l % 6 {
        | 0 => map_style = (11, 9, 8),
        | 1 => map_style = (11, 8, 9),
        | 2 => map_style = (9, 11, 8),
        | 3 => map_style = (9, 8, 11),
        | 4 => map_style = (8, 9, 11),
        | 5 => map_style = (8, 11, 9),
        | _ => map_style = (0, 10, 0)
    }
    let mut path1: Vec<u8> = Vec::with_capacity(map_style.0.into());
    let mut path2: Vec<u8> = Vec::with_capacity(map_style.1.into());
    let mut path3: Vec<u8> = Vec::with_capacity(map_style.2.into());
    path1 = add_path(map_style.0, l, 1);
    let mut i: u8 = 0;
    while i < map_style.0 {
        l = change_seed(l);
        i += 1;
    }
    i = 0;
    path2 = add_path(map_style.1, l, 2);
    while i < map_style.1 {
        l = change_seed(l);
        i += 1;
    }
    path3 = add_path(map_style.2, change_seed(l), 3);
    let f_map = Map {
        pa1: path1,
        pa2: path2,
        pa3: path3
    };
    f_map
}

fn add_path(length: u8, mut l2: u64, num: u8) -> Vec<u8> {
    let mut p1: Vec<u8> = Vec::with_capacity(length.into());
    l2 = change_seed(l2);
    let mut choice: u8 = (l2 % 7) as u8;
    p1.push(choice);
    if num == 1 {
        p1.push(1);
    }
    else {
        l2 = change_seed(l2);
        choice = (l2 % 7) as u8;
        p1.push(choice);
    }

    let mut i: u8 = 0;
    while i < (length - 3) {
        l2 = change_seed(l2);
        choice = (l2 % 7) as u8;
        p1.push(choice);
        i += 1;
    }
    if length == 11 {
        p1.push(7)
    }
    else {
        l2 = change_seed(l2);
        choice = (l2 % 7) as u8;
        p1.push(choice);
    }
    p1
}

fn change_seed(mut number: u64) -> u64 {
    number = number.overflowing_mul(3838528834).0;
    number = number.overflowing_add(873821).0;
    number
}

fn serialize_base_stuff(world_name: String, seed: u64, home: String) -> std::io::Result<()> {
    let mut file_not_ready = true;
    let mut i = 0;
    let mut world_nam3: String;
    let path = format!("{}/.fanterra/worlds", home);
    if Path::new(&path).exists() {
    }
    else {
        fs::create_dir_all(&path)?;
    }
    while file_not_ready {
        if i == 0 {
            world_nam3 = format!("{}/{}.fanterra", &path, world_name);
        }
        else {
            world_nam3 = format!("{}/{}({}).fanterra", &path, world_name, i);
        }
        match fs::read(&world_nam3) {
            | Ok(_) => i += 1,
            | Err(why) => {
                if i == 0 {
                    let y2 = why;
                    if i != 0 {
                        println!("{}", y2);
                    }
                    file_not_ready = false;
                }
                else {
                    file_not_ready = false;
                }
            },
        }
    }
    let actual_world_name;
    if i == 0 {
        world_nam3 = format!("{}/{}.fanterra", &path, world_name);
        actual_world_name = format!("{}.fanterra", world_name);
    }
    else {
        world_nam3 = format!("{}/{}({}).fanterra", &path, world_name, i);
        actual_world_name = format!("{}({}).fanterra", world_name, i);
    }
    println!("Your world name is {}.\n", actual_world_name);
    fs::write(world_nam3, seed.to_le_bytes())?;
    Ok(())
}

pub fn read_worlds(home: String) -> Vec<String> {
    let path = format!("{}/.fanterra/worlds", home);
    let entries = fs::read_dir(&path).unwrap();
    let mut v = Vec::new();
    let mut j = 0;
    for i in entries {
        j += 1;
        let j_s = format!("{} ", &j.to_string());
        let item = i.unwrap().path().display().to_string();
        let v2: Vec<&str> = item.split(".fanterra/worlds/").collect();
        let world_name_real = format!("{}", v2[v2.len() - 1].to_string());
        v.push(format!("{}{}\n", j_s, world_name_real));
    }

    v
}

fn read_file(world: &String, home: &String) -> u64 {
    let path = format!("{}/.fanterra/worlds/{}", home, world);
    let file = fs::read(path).unwrap();
    let bytes: [u8; 8] = [
        file[0], file[1], file[2], file[3], file[4], file[5], file[6], file[7]
    ];
    let seed = u64::from_le_bytes(bytes);
    seed
}

pub struct Game {
    pub seed: u64,
    pub pa1:  Vec<u8>,
    pub pa2:  Vec<u8>,
    pub pa3:  Vec<u8>
}

pub struct Map {
    pa1: Vec<u8>,
    pa2: Vec<u8>,
    pa3: Vec<u8>
}
