use std::io::{self, Write};

pub fn get_input() -> String {
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Error reading input");
    name.trim().to_string()
}