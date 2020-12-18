use std::iter::Peekable;
use std::slice::Iter;
use std::str::FromStr;

#[derive(Debug)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParenthesis,
    RightParenthesis,
}

struct AstNode {
    left_child: Option<AstNode>,
    right_child: Option<AstNode>,
    node_type: AstNodeType,
}

enum AstNodeType {
    Expr,
    Add,
    Number(i32),
}

fn main() -> Result<(), Error> {
    let input = "5 + 23";
    let tokens = tokenize(&input)?;
    dbg!(tokens);
    Ok(())
}

fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    input.trim().split(" ").try_fold(vec![], |mut tokens, str| {
        tokens.push(str.parse()?);
        Ok(tokens)
    })
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match i32::from_str_radix(s, 10) {
            Ok(number) => Self::Number(number),
            _ => match s {
                "+" => Self::Plus,
                "-" => Self::Minus,
                "*" => Self::Multiply,
                "/" => Self::Divide,
                _ => return Err(Error::UnknownToken(s.into())),
            },
        })
    }
}

#[derive(Debug)]
enum Error {
    UnknownToken(String),
}
