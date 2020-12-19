use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;
use crate::Token::Operator;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, Copy, Clone, PartialEq)]
enum Token {
    Number(i64),
    Operator(OperatorType),
    LeftParenthesis,
    RightParenthesis,
}

impl Token {
    fn value(&self) -> i64 {
        match self {
            Token::Number(value) => *value,
            _ => panic!("No value for token")
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum OperatorType {
    Plus,
    Minus,
    Multiply,
    Divide
}

fn main() -> Result<(), Error> {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);
    let result = reader.lines().fold(0, |acc, line| {
        let line = line.unwrap();
        let tokens = tokenize(&line).unwrap();
        let mut token_iterator = tokens.iter().peekable();
        acc + evaluate_expression(&mut token_iterator)
    });
    println!("{}", result);
    Ok(())
}

fn evaluate_expression(token_iterator: &mut Peekable<Iter<Token>>) -> i64 {
    let mut result = evaluate_term(token_iterator).value();
    while let Some(Token::Operator(OperatorType::Multiply)) | Some(Token::Operator(OperatorType::Divide)) = token_iterator.peek() {
        match token_iterator.peek() {
            Some(Token::Operator(OperatorType::Multiply)) => {
                eat_operator(OperatorType::Multiply, token_iterator);
                result = result * evaluate_term(token_iterator).value();
            },
            Some(Token::Operator(OperatorType::Divide)) => {
                eat_operator(OperatorType::Divide, token_iterator);
                result = result / evaluate_term(token_iterator).value();
            },
            _ => panic!("Unexpected token")
        }
    }

    result
}

fn evaluate_factor(token_iterator: &mut Peekable<Iter<Token>>) -> Token {
    let token = token_iterator.next();
    let mut result;
    match token {
        Some(Token::Number(value)) => result = Token::Number(*value),
        Some(Token::LeftParenthesis) => {
            result = Token::Number(evaluate_expression(token_iterator));
            eat_token(Token::RightParenthesis, token_iterator);
        }
        _ => panic!("Unexpected token")
    }

    result
}

fn evaluate_term(token_iterator: &mut Peekable<Iter<Token>>) -> Token {
    let mut result= evaluate_factor(token_iterator);

    while let Some(Token::Operator(OperatorType::Plus)) | Some(Token::Operator(OperatorType::Minus)) = token_iterator.peek() {
        match token_iterator.peek() {
            Some(Operator(OperatorType::Plus)) => {
                eat_operator(OperatorType::Plus, token_iterator);
                result = Token::Number(result.value() + evaluate_factor(token_iterator).value());
            },
            Some(Operator(OperatorType::Minus)) => {
                eat_operator(OperatorType::Minus, token_iterator);
                result = Token::Number(result.value() - evaluate_factor(token_iterator).value());
            },
            _ => panic!("Unexpected token")
        }
    }


    result
}

fn eat_token(token: Token, token_iterator: &mut Peekable<Iter<Token>>) {
    match token_iterator.next() {
        Some(t) => {
            if *t != token {
                panic!("Unexpected token")
            }
        },
        _ => panic!("Unexpected token")
    }
}

fn eat_number(token_iterator: &mut Peekable<Iter<Token>>) {
    match token_iterator.next() {
        Some(Token::Number(_)) => (),
        _ => panic!("Expected number"),
    }
}

fn eat_operator(operator_type: OperatorType, token_iterator: &mut Peekable<Iter<Token>>) {
    match token_iterator.next() {
        Some(Token::Operator(t)) => {
            if *t == operator_type {

            } else {
                panic!("Expected another operator")
            }
        },
        _ => panic!("Expected operator"),
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = vec![];
    let mut iterator = input.trim().bytes().peekable();
    while let Some(character) = iterator.peek() {
        if character == &b' ' {
            iterator.next();
            continue;
        } else if character >= &b'0' && character <= &b'9' {
            let character = iterator.next().unwrap();
            let mut number_str = String::from_utf8(vec![character]).unwrap();
            while let Some(c) = iterator.peek() {
                if c >= &b'0' && c <= &b'9' {
                    number_str += &*String::from_utf8(vec![*c]).unwrap();
                } else {
                    break;
                }
            }

            tokens.push(number_str.parse().unwrap());
        } else if [b'+', b'-', b'*', b'/', b'(', b')'].contains(character) {
            let character = iterator.next().unwrap();
            tokens.push(String::from_utf8(vec!(character)).unwrap().parse().unwrap());
        }
    }

    Ok(tokens)
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match i64::from_str_radix(s, 10) {
            Ok(number) => Self::Number(number),
            _ => match s {
                "+" => Self::Operator(OperatorType::Plus),
                "-" => Self::Operator(OperatorType::Minus),
                "*" => Self::Operator(OperatorType::Multiply),
                "/" => Self::Operator(OperatorType::Divide),
                "(" => Self::LeftParenthesis,
                ")" => Self::RightParenthesis,
                _ => return Err(Error::UnknownToken(s.into())),
            },
        })
    }
}

#[derive(Debug)]
enum Error {
    UnknownToken(String),
}
