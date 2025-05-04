use std::{
    fmt::{self, Display},
    fs,
};

#[derive(Copy, Clone)]
struct RgbColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for RgbColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB({}, {}, {})", self.red, self.green, self.blue)
    }
}

const PATH_TO_INPUT_FILE: &str = r"inf_emelt_2023_tavasz\kep.txt";

fn main() {
    println!("--- INFORMATIKA EMELT ÉRETTSÉGI - 2023. TAVASZ - RGB SZÍNEK ---");
    let image = feladat1();
    feladat2();
    feladat3();
    feladat4();
    feladat5();
    feladat6();
}

fn feladat1() -> Vec<Vec<RgbColor>> {
    println!("\n1. feladat");

    let binding =
        fs::read_to_string(PATH_TO_INPUT_FILE).expect("should have been able to read the file");
    let rows = binding.lines();

    let mut image: Vec<Vec<RgbColor>> = vec![
        vec![
            RgbColor {
                red: 0,
                green: 0,
                blue: 0,
            };
            640
        ];
        360
    ];

    let mut row_num: usize = 0;
    let mut column_num: usize;
    let mut color_index: usize;

    for row in rows {
        color_index = 0;
        column_num = 0;

        for column in row.split_whitespace() {
            let value: u8 = column
                .parse::<u8>()
                .expect("should have been able to parse value");

            if color_index == 3 {
                color_index = 0;
                column_num += 1;
            }

            /* println!("value: {value}, row_num: {row_num}, column_num: {column_num}, color_index: {color_index}"); */

            match color_index {
                0 => {
                    image[row_num][column_num].red = value;
                    color_index += 1;
                }
                1 => {
                    image[row_num][column_num].green = value;
                    color_index += 1;
                }
                2 => {
                    image[row_num][column_num].blue = value;
                    color_index += 1;
                }
                _ => eprintln!("HIBA"),
            }
        }
        row_num += 1;
    }

    println!("Sikeresen beolvasva.");
    return image;
}

fn feladat2() {}
fn feladat3() {}
fn feladat4() {}
fn feladat5() {}
fn feladat6() {}
