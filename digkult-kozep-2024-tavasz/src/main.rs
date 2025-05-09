use std::fs;

const INPUT_PATH: &str = "digkult-kozep-2024-tavasz/dobasok.txt";

fn main() {
    println!("--- Digitális kultúra, közép - 2024. tavasz - Létra ---");

    let mut visszalepesek = 0;

    let dobasok = beolvasas();

    println!("\n2. feladat");
    let mut current_position = 0;
    for dobas in dobasok {
        if (current_position + dobas) % 10 == 0 {
            visszalepesek += 1;
            current_position += dobas - 3
        } else {
            current_position += dobas
        }

        print!("{} ", current_position)
    }
    println!();

    println!("\n3. feladat");
    println!("A játék során {} alkalommal lépett létrára.", visszalepesek);

    println!("\n4. feladat");
    if current_position >= 45 {
        println!("A játékot befejezte.")
    } else {
        println!("A játékot abbahagyta.")
    }
}

fn beolvasas() -> Vec<i32> {
    let mut to_return = vec![];

    let text = fs::read_to_string(INPUT_PATH).expect("Hiba történt a fájl beolvasásakor.");
    for value in text.split(", ") {
        to_return.push(
            value
                .trim_ascii()
                .parse()
                .expect("Hiba történt egy szám értelmezésekor."),
        )
    }

    to_return
}
