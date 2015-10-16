use std::io::prelude::*;
use std::io;

extern crate regex;
use regex::Regex;

enum Token {
    LParen,
    RParen,
    Add,
    Subtract,
    Multiply,
    Divide,
    Number,
    Variable,
    Assign,
    Invalid
}

fn define_token(token: &str) -> Token {
    let re_var = Regex::new(r"^([a-z]|[A-z])(\d|[a-z]|[A-Z])*$").unwrap();
    let re_num = Regex::new(r"^\d*\.\d*$").unwrap();
    if re_var.is_match(token) {
        return Token::Variable;
    }
    else if re_num.is_match(token) {
        return Token::Number;
    }
    else {
        return match token {
            "(" => Token::LParen,
            ")" => Token::RParen,
            "+" => Token::Add,
            "-" => Token::Subtract,
            "*" => Token::Multiply,
            "/" => Token::Divide,
            "=" => Token::Assign,
            _ => Token::Invalid
        };
    }
}

fn define_tokens(tokens: Vec<String>) -> Vec<Token> {
    let mut defined = Vec::new();
    for token in tokens {
        defined.push(define_token(token.as_ref()));
    }
    return defined;
}


fn tokenize_input(input: &str) -> Vec<String> {
    let chars = input.chars();
    let mut tokens = Vec::new();
    let mut next: String = String::new();
    
    let re_splitter = Regex::new(r"(\(|\)|\+|-|\*|/|=)").unwrap();
    let re_whitespace = Regex::new(r"\s").unwrap();
    for c in chars {
        let mut c_string = String::new();
        c_string.push(c);
        let c_str = c_string.as_ref();
        if re_splitter.is_match(c_str) {
            if !next.is_empty() {
                tokens.push(String::from(next.as_ref()));
                next.clear();
            }
            next.push(c);
            tokens.push(String::from(next.as_ref()));
            next.clear();
        }
        else if re_whitespace.is_match(c_str) {
            if !next.is_empty() {
                tokens.push(String::from(next.as_ref()));
                next.clear();
            }
        }
        else {
            next.push(c);
        }
    }
    if !next.is_empty() {
        tokens.push(String::from(next.as_ref()));
    }
    return tokens;
}

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
    let input = get_user_input("> ");
    let undefined_tokens = tokenize_input(input.as_ref());
    let defined_tokens = define_tokens(undefined_tokens);
    for token in defined_tokens {
        match token {
            Token::LParen => println!("Left paren"),
            Token::RParen => println!("Right paren"),
            Token::Add => println!("Add"),
            Token::Subtract=> println!("Subtract"),
            Token::Multiply=> println!("Multiply"),
            Token::Divide=> println!("Divide"),
            Token::Number=> println!("Number"),
            Token::Variable=> println!("Variable"),
            Token::Assign=> println!("Assign"),
            Token::Invalid=> println!("Invalid")
        }
    }
}
