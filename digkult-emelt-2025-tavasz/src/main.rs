use std::fmt;
use std::fs;

mod html_table;

const FILE_PATH: &str = "digkult-emelt-2025-tavasz/kiadas.txt";

#[derive(Debug)]
struct Book {
    year: i32,
    quarter: i32,
    hungarian: bool,
    desc: String,
    copies: i32,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{};{};{};{};{}",
            self.year,
            self.quarter,
            if self.hungarian { "ma" } else { "kf" },
            self.desc,
            self.copies
        )
    }
}

fn read_file() -> Vec<Book> {
    let mut books: Vec<Book> = vec![];

    let file = fs::read_to_string(FILE_PATH).expect("Hiba történt a fájl beolvasása során.");
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
    let x = html_table::HtmlTable {
        header: vec![
            String::from("Év"),
            String::from("Magyar kiadás"),
            String::from("Magyar példányszám"),
            String::from("Külföldi kiadás"),
            String::from("Külföldi példányszám"),
        ],
        rows: vec![
            vec![
                String::from("2020"),
                String::from("45"),
                String::from("834005"),
                String::from("29"),
                String::from("416000"),
            ],
            vec![
                String::from("2021"),
                String::from("49"),
                String::from("779130"),
                String::from("52"),
                String::from("736900"),
            ],
            vec![
                String::from("2022"),
                String::from("63"),
                String::from("1115210"),
                String::from("42"),
                String::from("649639"),
            ],
            vec![
                String::from("2023"),
                String::from("41"),
                String::from("625185"),
                String::from("40"),
                String::from("536000"),
            ],
        ],
    };

    let html = x.generate_html();
    println!("{}", html)
}
