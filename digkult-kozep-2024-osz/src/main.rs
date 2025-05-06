use std::{fs, usize};
use user_input::take_and_convert_user_input_aggressively;

const INPUT_PATH: &str = "digkult-kozep-2024-osz/uvegek.txt";
fn main() {
    println!("--- Digitális kultúra, közép - 2024. ősz - Befőzés ---");

    println!("\n1. feladat");
    let jars = feladat1();
    println!("Sikeresen beolvasva.");

    println!("\n2. feladat");
    let amount = feladat2();

    println!("\n3. feladat");
    let (max_volume, biggest_jar) = feladat3(jars);
    println!(
        "A legnagyobb üveg {} dl-es és a {}. a sorban.",
        max_volume, biggest_jar
    );

    println!("\n4. feladat");
    let fits: bool = feladat4(jars, amount);
    println!(
        "{}",
        if fits {
            "Elegendő üveg volt."
        } else {
            "Nem volt elegendő üveg."
        }
    )
}

/// A megadott 15 számot tárolja el a programban egy megfelelő adatszerkezetben! A 15 szám rendelkezésre áll az <code>uvegek.txt</code> állományban, amelyből azok a program kódjába átmásolhatók.
fn feladat1() -> [i32; 15] {
    let mut to_return = [-1; 15];

    let text = fs::read_to_string(INPUT_PATH).expect("Hiba történt a fájl beolvasásakor.");
    for (idx, value) in text.split(", ").enumerate() {
        to_return[idx] = value
            .trim_ascii()
            .parse()
            .expect("Hiba történt egy szám értelmezésekor.");
    }

    to_return
}

/// Kérje be a mintának megfelelően, és tárolja el, hogy Mari néni hány deciliter lekvárt (<code>L</code>) főz be, ahol <code>L</code> értéke <code>0 &lt; L &le; 200</code>!
fn feladat2() -> i32 {
    take_and_convert_user_input_aggressively::<i32>(
        "Adja meg a befőzött lekvár mennyiségét: (dl) ",
        "Hibás számot adott meg!",
    )
}

/// Az üvegek űrtartalma alapján határozza meg, hogy a legnagyobb üveg hány deciliteres és hányadik a sorban! Ha több ilyen van, akkor az elsőt adja meg!
fn feladat3(jars: [i32; 15]) -> (i32, usize) {
    let mut max_volume: i32 = jars[0];
    for jar in jars {
        if jar > max_volume {
            max_volume = jar;
        }
    }

    let mut biggest_jar: usize = 0;
    for i in 0..15 {
        if jars[i] == max_volume {
            biggest_jar = i + 1 // vissza-eltoljuk az indexet
        }
    }

    (max_volume, biggest_jar)
}

/// Írassa ki a képernyőre, hogy Mari néni <code>L</code> deciliter befőzött lekvárja elfér-e az üvegekben! Ha az üveg mennyiség elegendő, akkor írja ki, hogy „Elegendő üveg volt.”, különben azt, hogy „Maradt lekvár.”!
fn feladat4(jars: [i32; 15], amount: i32) -> bool {
    amount <= jars.iter().sum()
}
