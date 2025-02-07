use std::io::{self, Write};
use colored::Colorize;

pub fn get_entity_name() -> String {
    print!("{}", "Enter the entity name: ".blue().bold());
    io::stdout().flush().unwrap();
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Error reading input");
    name.trim().to_string()
}