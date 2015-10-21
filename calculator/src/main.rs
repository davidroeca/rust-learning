use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::collections::HashMap;

extern crate regex;
use regex::Regex;

enum Token {
    LParen,
    RParen,
    Add,
    Subtract,
    Multiply,
    Divide,
    Number(f64),
    Variable(String),
    Assign,
    Invalid
}

enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide
}

enum Expression {
    Single(f64),
    Full(Box<Expression>, BinaryOp, Box<Expression>)
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

fn define_tokens(tokens: Vec<String>) -> Vec<Token> {
    let mut defined = Vec::new();
    for token in tokens {
        defined.push(define_token(token.as_ref()));
    }
    return defined;
}

fn define_token(token: &str) -> Token {
    let re_var = Regex::new(r"^([a-z]|[A-z])(\d|[a-z]|[A-Z])*$").unwrap();
    let re_num = Regex::new(r"^\d*\.??\d+$").unwrap();
    if re_var.is_match(token) {
        return Token::Variable(token.to_string());
    }
    else if re_num.is_match(token) {
        return Token::Number(token.parse::<f64>().ok().expect("err"));
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

fn parse_tokens(tokens: Vec<Token>, vars: &mut HashMap<String, f64>) -> Expression {
    let length = tokens.len();
    let max_index = length - 1;
    fn parse_expression(index: usize, max_index: usize, tokens: Vec<Token>,
                        vars: &mut HashMap<String, f64>) -> (Expression, usize) {
        if index > max_index {
            panic!("Index out of range");
        }
        let (LHS, op_index) = match tokens[index] {
            Token::LParen => parse_expression(index + 1, max_index, tokens, vars),
            Token::Number(n) => (Expression::Single(n), index + 1),
            Token::Variable(v) => (Expression::Single(vars[v]), index + 1),
            _ => panic!("Invalid Token")
        };
        if op_index > max_index {
            panic!("Index out of range");
        }
        let (OP, rhs_index) = (match tokens[op_index] {
            Token::Add => BinaryOp::Add,
            Token::Subtract => BinaryOp::Subtract,
            Token::Divide => BinaryOp::Divide,
            Token::Multiply => BinaryOp::Multiply
        }, op_index + 1);
        if rhs_index > max_index {
            panic!("Index out of range");
        }
        let (RHS, next_index) = match tokens[rhs_index] {
            Token::LParen => parse_expression(rhs_index + 1, max_index, tokens, vars),
            Token::Number(n) => (Expression::Single(n), op_index + 1),
            Token::Variable(v) => (Expression::Single(vars.get(v)), op_index + 1),
            _ => panic!("Invalid Token")
        };
        if next_index > max_index {
            panic!("Index out of range");
        }
        match tokens[next_index] {
            Token::RParen => (),
            _ => panic!("Right parenthesis expected")
        };
        return (Expression::Full(Box::new(LHS), OP, Box::new(RHS)), next_index + 1);
    }
    let (expression, _) = parse_expression(0, max_index, tokens, vars);
    expression
    //for i in 0..length {
        //match tokens[i] {
            //Token::LParen => println!("Left paren"),
            //Token::RParen => println!("Right paren"),
            //Token::Add => println!("Add"),
            //Token::Subtract => println!("Subtract"),
            //Token::Multiply => println!("Multiply"),
            //Token::Divide => println!("Divide"),
            //Token::Number(v) => println!("Number: {}", v),
            //Token::Variable(ref n) => println!("Variable: {}", n),
            //Token::Assign => println!("Assign"),
            //Token::Invalid => println!("Invalid")
        //}
    //}
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
        let undefined_tokens = tokenize_input(input.as_ref());
        let defined_tokens = define_tokens(undefined_tokens);
        parse_tokens(defined_tokens, &mut vars);
    }
}
