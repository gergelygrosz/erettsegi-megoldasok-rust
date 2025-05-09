use std::fs;

const INPUT_PATH: &str = "digkult-kozep-2023-osz\\tomeg.txt";

fn main() {
    println!("--- Digitális kultúra, közép - 2023. ősz - Szállítás ---");

    let weights = read_file();

    println!("\n2. feladat");
    println!(
        "A tárgyak tömegének összege: {} kg",
        weights.iter().sum::<i32>()
    );

    println!("\n3. feladat");

    let mut boxes: Vec<i32> = vec![];
    let mut current_box: Vec<i32> = vec![];

    for weight in weights.clone() {
        // ha az adott doboz már túl nagy lenne még egy tárggyal, lezárjuk
        if current_box.iter().sum::<i32>() + weight > 20 {
            boxes.push(current_box.iter().sum::<i32>());
            current_box.clear();
        }

        current_box.push(weight);
    }
    // a sor végére érve a legutolsó dobozt is lezárjuk
    boxes.push(current_box.iter().sum());

    print!("A dobozok tartalmának tömege (kg): ");
    for doboz in boxes.clone() {
        print!("{} ", doboz)
    }
    println!("\nA szükséges dobozok száma: {}", boxes.len())
}

fn read_file() -> Vec<i32> {
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
