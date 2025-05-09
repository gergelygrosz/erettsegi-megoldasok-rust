use std::{
    io::{Write, stdin, stdout},
    str::FromStr,
};

/// Reads lines from stdin until the input can be parsed to `T`.
/// Displays `prompt`, and asks for input.
/// If the input can't be parsed to T, displays `error_msg`.
///
/// # Examples
/// ```
/// let num = user_input::take_and_convert_user_input_aggressively::<i32>("num=", "please type a number");
/// ```
pub fn take_and_convert_user_input_aggressively<T: FromStr>(prompt: &str, error_msg: &str) -> T {
    loop {
        print!("{}", prompt);
        stdout()
            .flush()
            .expect("should have been able to flush stdout");

        let mut target = String::new();
        stdin()
            .read_line(&mut target)
            .expect("should have been able to read user input");

        match target.trim_ascii().parse::<T>() {
            Ok(x) => return x,
            Err(_) => {
                eprintln!("{}", error_msg);
            }
        }
    }
}
