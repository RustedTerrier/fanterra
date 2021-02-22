use rand::Rng;
use std::io;

pub fn create_world() {
    let seed = create_seed();
    println!("{}", seed);
    let world_name = create_name(&seed, 5);
    println!("{}", world_name);
}

pub fn create_name(seed: &String, len: u32) -> String {
    let vowels: [char; 5] = ['A', 'E', 'I', 'O', 'U'];
    //Screw y
    let consonants: [char; 20] = [
        'Q', 'W', 'R', 'T', 'P', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'Z', 'X', 'C', 'V', 'B',
        'N', 'M',
    ];
    let mut number = seed.parse::<f32>().unwrap();
    number = number * 0.03;
    number = number / 6.1;
    let snum: &String = &number.to_string();
    let cnum: &char = &snum.chars().nth(0).unwrap();
    let mut name = add_vowel("".to_string(), &cnum, &vowels);
    if len == 5 {
        name = add_consonant(name, &cnum, &consonants);
        number = number * (3.284 * &cnum.to_string().parse::<f32>().unwrap()) / 5.76;
        name = add_consonant(
            name,
            &number.to_string().chars().nth(1).unwrap(),
            &consonants,
        );
        number = number * (3.284 * &cnum.to_string().parse::<f32>().unwrap()) / 5.76;
        name = add_vowel(name, &number.to_string().chars().nth(2).unwrap(), &vowels);
        number = number * (3.284 * &cnum.to_string().parse::<f32>().unwrap()) / 5.76;
        name = add_consonant(
            name,
            &number.to_string().chars().nth(3).unwrap(),
            &consonants,
        );
    }
    name
}
fn add_vowel(word: String, cnum: &char, vowels: &[char; 5]) -> String {
    let name = format!(
        "{}{}",
        word,
        vowels[(&cnum.to_string().parse::<u8>().unwrap() % 5) as usize].to_string()
    );
    name
}

fn add_consonant(word: String, cnum: &char, consonants: &[char; 20]) -> String {
    let name = format!(
        "{}{}",
        word,
        consonants[(&cnum.to_string().parse::<u8>().unwrap() % 20) as usize].to_string()
    );
    name
}

pub fn create_seed() -> String {
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
