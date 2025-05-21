use std::fmt;

pub struct Book {
    pub year: i32,
    pub quarter: i32,
    pub hungarian: bool,
    pub desc: String,
    pub copies: i32,
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
