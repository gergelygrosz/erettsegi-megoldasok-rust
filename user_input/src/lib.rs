use std::{
    io::{Write, stdin, stdout},
    str::FromStr,
};

pub fn take_and_convert_user_input_aggressively<T: FromStr>(prompt: &str, error_msg: &str) -> T {
    loop {
        print!("{}", prompt);
        stdout()
            .flush()
            .expect("should have been able to flush stdout");

        let mut target = String::new();
        stdin()
            .read_line(&mut target)
            .expect("Hiba történt a bemenet olvasása közben.");

        match target.trim_ascii().parse::<T>() {
            Ok(x) => return x,
            Err(_) => {
                println!("{}", error_msg);
            }
        }
    }
}
