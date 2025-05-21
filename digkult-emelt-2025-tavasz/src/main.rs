use std::fs;

use book::Book;
use html_table::Table;

mod book;
mod html_table;

const INPUT_FILE: &str = "digkult-emelt-2025-tavasz/kiadas.txt";
const OUTPUT_FILE: &str = "digkult-emelt-2025-tavasz/tabla.html";
const FIRST_YEAR: i32 = 2020;
const LAST_YEAR: i32 = 2023;

fn read_file() -> Vec<Book> {
    let mut books: Vec<Book> = vec![];

    let file = fs::read_to_string(INPUT_FILE).expect("Hiba történt a fájl beolvasása során.");
    for line in file.lines() {
        let line: Vec<&str> = line.trim().split(';').collect();

        let book = Book {
            year: str::parse(line[0]).expect("Hiba történt a fájl beolvasása során."),
            quarter: str::parse(line[1]).expect("Hiba történt a fájl beolvasása során."),
            hungarian: match line[2] {
                "ma" => true,
                _ => false,
            },
            desc: line[3].to_string(),
            copies: str::parse(line[4]).expect("Hiba történt a fájl beolvasása során."),
        };

        books.push(book);
    }

    books
}

fn main() {
    println!("Digitális kultúra, emelt - 2025. tavasz - Könyvkiadás");

    let books = read_file();

    println!("\n2. feladat");
    let author: String = user_input::take_and_convert_user_input_aggressively(
        "Szerző: ",
        "Hiba történt. Próbálja újra.",
    );
    let num_books_by_author = books
        .iter()
        .filter(|book| book.desc.contains(&author))
        .count();
    println!("{} könyvkiadás", num_books_by_author);

    println!("\n3. feladat");
    let max_copies = books.iter().max_by_key(|book| book.copies).unwrap().copies;
    let num_max_copies = books
        .iter()
        .filter(|book| book.copies == max_copies)
        .count();
    println!(
        "Legnagyobb példányszám: {}, előfordul {} alkalommal",
        max_copies, num_max_copies
    );

    println!("\n4. feladat");
    let big_foreign_books = books
        .iter()
        .filter(|book| !book.hungarian && book.copies > 40000)
        .collect::<Vec<&Book>>();
    let big_foreign_book = big_foreign_books.first().unwrap();
    println!(
        "{}/{}. {}",
        big_foreign_book.year, big_foreign_book.quarter, big_foreign_book.desc
    );

    println!("\n5. feladat");
    let header: Vec<String> = vec![
        String::from("Év"),
        String::from("Magyar kiadás"),
        String::from("Magyar példányszám"),
        String::from("Külföldi kiadás"),
        String::from("Külföldi példányszám"),
    ];

    let mut body: Vec<Vec<i32>> = Vec::new();
    for year in FIRST_YEAR..(LAST_YEAR + 1) {
        let current_row: Vec<i32> = vec![
            year,
            books
                .iter()
                .filter(|b| b.year == year && b.hungarian)
                .count() as i32, // Magyar kiadás
            books
                .iter()
                .filter_map(|b| {
                    if b.year == year && b.hungarian {
                        Some(b.copies)
                    } else {
                        None
                    }
                })
                .sum::<i32>() as i32, // Magyar példányszám
            books
                .iter()
                .filter(|b| b.year == year && !b.hungarian)
                .count() as i32, // Külföldi kiadás
            books
                .iter()
                .filter_map(|b| {
                    if b.year == year && !b.hungarian {
                        Some(b.copies)
                    } else {
                        None
                    }
                })
                .sum::<i32>() as i32, // Külföldi példányszám
        ];
        body.push(current_row);
    }

    let table = Table {
        headers: header,
        body: body,
    };

    print!("{}", table.generate_string());
    fs::write(OUTPUT_FILE, table.generate_html()).expect("Hiba történt a fájl kiírása során.");

    println!("\n6. feladat");
    let mut duplicate_descs: Vec<String> = vec![];

    for current_book in &books {
        if duplicate_descs.contains(&current_book.desc) {
            continue;
        }

        if books
            .iter()
            .filter_map(|given_book| {
                if given_book.desc == current_book.desc && given_book.copies > current_book.copies {
                    Some(&given_book.desc)
                } else {
                    None
                }
            })
            .count()
            >= 2
        {
            duplicate_descs.push(current_book.desc.clone());
        }
    }

    println!("Legalább kétszer, nagyobb példányszámban újra kiadott könyvek:");
    for desc in duplicate_descs {
        println!("{}", desc)
    }
}
