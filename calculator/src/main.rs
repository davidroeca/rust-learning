use std::io::prelude::*;
use std::io;
use std::collections::HashMap;


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

fn tokenize_input(input_chars: Vec<char>) {
    let possible_tokens = vec!['(', ')', '-', '+'];
    for c in input_chars {
        if possible_tokens.contains(&c) {
            println!("{}", c);
        }
        else {
            println!("Not a token.");
        }
    }
}

fn main() {
    let input = get_user_input("> ");
    println!("-{}-", input);
    let input_chars: Vec<char> = input.chars().collect();
    tokenize_input(input_chars);
}
