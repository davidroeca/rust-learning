use std::ops::Deref;
use std::collections::HashMap;

use regex::Regex;

pub enum Token {
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
    Full(Box<Expression>, BinaryOp, Box<Expression>),
    Not
}

pub fn tokenize_input(input: &str) -> Vec<String> {
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


pub fn define_tokens(tokens: Vec<String>) -> Vec<Token> {
    let mut defined = Vec::new();
    for token in tokens {
        defined.push(define_token(token.as_ref()));
    }
    return defined;
}

pub fn parse_tokens(tokens: &Vec<Token>,
                vars: &mut HashMap<String, f64>) -> Option<f64> {
    let length = tokens.len();
    let max_index = length - 1;

    fn parse_expression(index: usize, max_index: usize, tokens: &Vec<Token>,
                        vars: &HashMap<String, f64>) -> (Expression, usize) {
        let incorrect_syntax = (Expression::Not, max_index + 1);
        if index > max_index {
            return incorrect_syntax;
        }
        let (lhs, op_index) = match tokens[index] {
            Token::Number(n) => (Expression::Single(n), index + 1),
            Token::Variable(ref v) => analyze_variable(v, &vars, index,
                                                       max_index),
            Token::LParen => parse_expression(index + 1, max_index, tokens,
                                              vars),
            _ => return incorrect_syntax
        };
        if op_index > max_index {
            return incorrect_syntax;
        }
        let (op, rhs_index) = (match tokens[op_index] {
            Token::Add => BinaryOp::Add,
            Token::Subtract => BinaryOp::Subtract,
            Token::Divide => BinaryOp::Divide,
            Token::Multiply => BinaryOp::Multiply,
            Token::RParen => return (lhs, op_index + 1),
            _ => return incorrect_syntax
        }, op_index + 1);
        if rhs_index > max_index {
            return incorrect_syntax;
        }
        let (rhs, next_index) = match tokens[rhs_index] {
            Token::Number(n) => (Expression::Single(n), rhs_index + 1),
            Token::Variable(ref v) => analyze_variable(v,
                                                              &vars,
                                                              rhs_index,
                                                              max_index),
            Token::LParen => parse_expression(rhs_index + 1, max_index, tokens,
                                              vars),
            _ => return incorrect_syntax
        };
        //if next_index > max_index {
            //return incorrect_syntax;
        //}
        //match tokens[next_index] {
            //Token::RParen => (),
            //_ => return incorrect_syntax
        //};
        return (Expression::Full(Box::new(lhs), op, Box::new(rhs)),
                next_index);
    }
    fn parse_assignment(tokens: &Vec<Token>, vars: &mut HashMap<String, f64>,
                        max_index: usize) -> Option<f64> {
        let var = match tokens[0] {
            Token::Variable(ref v) => v,
            _ => return None
        };
        match tokens[1] {
            Token::Assign => (),
            _ => return None
        };
        let (expression, _) = parse_expression(2, max_index, tokens, vars);
        match eval_expr(&expression) {
            Some(num) => vars.insert(String::from(var.as_ref()), num),
            None => None
        }
    }
    match parse_assignment(tokens, vars, max_index) {
        Some(num) => return Some(num),
        _ => ()
    };
    let (expression, _) = parse_expression(0, max_index, tokens, vars);
    eval_expr(&expression)
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

fn eval_expr(expr: &Expression) -> Option<f64> {
    match *expr {
        Expression::Single(val) => return Some(val),
        Expression::Full(ref e1, ref op, ref e2) => {
            let e1_val = match eval_expr(e1.deref()) {
                Some(val1) => val1,
                None => return None
            };
            let e2_val = match eval_expr(e2.deref()) {
                Some(val2) => val2,
                None => return None
            };
            return match *op {
                BinaryOp::Add => Some(e1_val + e2_val),
                BinaryOp::Subtract => Some(e1_val - e2_val),
                BinaryOp::Multiply => Some(e1_val * e2_val),
                BinaryOp::Divide => Some(e1_val / e2_val)
            }
        },
        Expression::Not => return None
    };
}

fn analyze_variable(var: &String,
                    vars: &HashMap<String, f64>,
                    index: usize,
                    max_index: usize) -> (Expression, usize) {
    return match vars.get(var) {
        Some(num) => (Expression::Single(*num), index + 1),
        None => (Expression::Not, max_index + 1)
    }
}
