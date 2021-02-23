use rand::Rng;
use std::io;

pub fn create_world() {
    let seed = create_seed();
    println!("{}", seed);
    let world_name = create_name(&seed, 5);
    println!("{}", world_name);
}

fn create_name(seed: &String, len: u32) -> String {
    let vowels: [char; 5] = ['A', 'E', 'I', 'O', 'U'];
    //Screw y
    let consonants: [char; 19] = [
        'W', 'R', 'T', 'P', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'Z', 'X', 'C', 'V', 'B', 'N',
        'M',
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
        seed = &seed2[0..seed2.len() - 1];
    } else {
        while seed2.len() < 12 {
            seed2 = format!("{}{}", seed2, rand::thread_rng().gen_range(0, 10));
        }
        seed = &seed2[0..];
    }

    seed.to_string()
}

fn create_map(seed: &String) {
    let mut l: u64 = change_seed(seed.parse::<u64>().unwrap());
    l = change_seed(p1);
    l = change_seed(p1);
    l = change_seed(p1);
    let map_style: (u8, u8, u8);
    match (l % 6) {
        0 => map_style = (11, 9, 8),
        1 => map_style = (11, 8, 9),
        2 => map_style = (9, 11, 8),
        3 => map_style = (9, 8, 11),
        4 => map_style = (8, 9, 11),
        5 => map_style = (8, 11, 9),
        _ => map_style = (0, 10, 0),
    }
    let p1 = Vec::with_capacity(128);
}

fn change_seed(mut number: u64) -> u64 {
    number = number.overflowing_mul(3838528).0;
    number = number.overflowing_add(873821).0 / 582;
    number
}
