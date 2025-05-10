use rand::Rng;
use std::fs;
use user_input::take_and_convert_user_input_aggressively;

const INPUT_PATH: &str = "digkult-kozep-2023-tavasz\\szavak.txt";

fn main() {
    println!("--- Digitális kultúra, közép - 2023. tavasz - Kitaláló ---");

    let words = read_file();
    let secret_number = rand::rng().random_range(1..=words.len());
    let chosen_word = &words[secret_number - 1];

    let mut count = 1;
    loop {
        if game_loop(chosen_word, count) {
            break;
        }

        count += 1
    }
}

fn read_file() -> Vec<String> {
    let mut to_return: Vec<String> = vec![];

    let text = fs::read_to_string(INPUT_PATH).expect("Hiba történt a fájl beolvasásakor.");
    for word in text.split(", ") {
        to_return.push(word.trim_ascii().trim_matches('"').to_string())
    }

    to_return
}

fn game_loop(word: &String, count: i32) -> bool {
    println!();

    let input = take_and_convert_user_input_aggressively::<String>(
        "Kérem a tippet: ",
        "Egy szót adjon meg!",
    );

    if input == "stop" {
        return true;
    }

    if input == *word {
        println!("{} tippeléssel sikerült kitalálni.", count);
        return true;
    }

    print!("Az eredmény: ");
    for (idx, c) in input.chars().enumerate() {
        if c == word.chars().nth(idx).expect("Hiba") {
            print!("{}", c)
        } else {
            print!(".")
        }
    }
    println!();

    false
}
