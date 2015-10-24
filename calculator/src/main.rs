use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

extern crate regex;
mod parser;

fn prompt(input_str: &str) {
    let input_string = String::from(input_str);
    io::stdout().write_all(input_string.as_bytes()).ok().unwrap();
    io::stdout().flush().ok().unwrap();
}

fn get_user_input(input_str: &str) -> String {
    prompt(input_str);
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Error");
    String::from(input.trim())
}

fn main() {
    let mut vars: HashMap<String, f64> = HashMap::new();
    loop {
        let input = get_user_input("> ");
        if input == "help" {
            println!("Help Prompt");
        }
        else if input == "quit" {
            break;
        }
        let undefined_tokens = parser::tokenize_input(input.as_ref());
        let defined_tokens = parser::define_tokens(undefined_tokens);
        let value = parser::parse_tokens(&defined_tokens, &mut vars);
        match value {
            Some(val) => println!("{}", val),
            None => println!("Something went wrong")
        };
    }
}
