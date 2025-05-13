use std::collections::HashMap;

use user_input::take_and_convert_user_input_aggressively;

fn main() {
    println!("--- Digitális kultúra, közép - 2025. tavasz - Kihívás ---");

    let mut distance_by_activity: HashMap<char, i32> = HashMap::new();
    distance_by_activity.insert('U', 1);
    distance_by_activity.insert('G', 1);
    distance_by_activity.insert('F', 2);
    distance_by_activity.insert('K', 10);

    println!("\n1. feladat");
    let input: String = take_and_convert_user_input_aggressively::<String>(
        "Adja meg az aktivitását: ",
        "Hiba történt a beolvasás közben. Próbálja újra!",
    )
    .trim()
    .to_uppercase();
    let user_activity: Vec<char> = input.chars().collect();

    println!("\n2. feladat");
    let mut total_distance: i32 = 0;
    for activity in &user_activity {
        total_distance += distance_by_activity.get(&activity).unwrap_or(&0);
    }
    println!("Az elért távolság: {} km.", total_distance);

    println!("\n3. feladat");
    let mut every_activity: bool = true;
    for activity in distance_by_activity.keys() {
        if !user_activity.contains(&activity) {
            every_activity = false;
            break;
        }
    }
    if every_activity {
        println!("Bravó! Jutalma még 10 km.");
        total_distance += 10;
    } else {
        println!("Nem jár jutalom.")
    }

    println!("\n4. feladat");
    print!("Eredménye: {} km. ", total_distance);
    if total_distance >= 40 {
        println!("Gratulálok, kihívás teljesítve!")
    } else {
        println!("Legközelebb sikerül!")
    }
}
