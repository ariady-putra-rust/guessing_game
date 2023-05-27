use std::io::{self, Write};

pub fn print_flush(s: &str) {
    print!("{}", s);
    io::stdout().flush().expect("Failed to flush!")
}
