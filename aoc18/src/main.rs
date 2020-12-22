use std::fs;

fn main() {
    println!("part 1: {}", solve(&parse_part1));
    println!("part 2: {}", solve(&parse_part2));
}

#[derive(Debug)]
enum Token {
    Int(i64),
    OpenParen,
    CloseParen,
    Plus,
    Times,
}

fn tokenize(line: &str) -> Vec<Token> {
    line.replace(" ", "")
        .chars()
        .map(|c| match c {
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '+' => Token::Plus,
            '*' => Token::Times,
            _ => {
                if c.is_digit(10) {
                    Token::Int(String::from(c).parse::<i64>().unwrap())
                } else {
                    panic!("parsing error")
                }
            }
        })
        .collect()
}

fn solve<F>(parse_fn: F) -> i64
where
    F: Fn(&mut Vec<Token>) -> i64,
{
    let mut sum = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let mut tokens = tokenize(line);
        sum += parse_fn(&mut tokens)
    }
    sum
}

fn parse_part1(tokens: &mut Vec<Token>) -> i64 {
    let val = match tokens.pop() {
        Some(Token::Int(val)) => val,
        Some(Token::CloseParen) => parse_part1(tokens),
        t => panic!("parsing error: {:?}", t),
    };
    match tokens.pop() {
        None | Some(Token::OpenParen) => val,
        Some(Token::Plus) => val + parse_part1(tokens),
        Some(Token::Times) => val * parse_part1(tokens),
        t => panic!("parsing error: {:?}", t),
    }
}

fn parse_part2(tokens: &mut Vec<Token>) -> i64 {
    let mut sum = 0;
    loop {
        sum += match tokens.pop() {
            Some(Token::Int(val)) => val,
            Some(Token::CloseParen) => parse_part2(tokens),
            t => panic!("parsing error: {:?}", t),
        };
        match tokens.pop() {
            None | Some(Token::OpenParen) => {
                return sum;
            }
            Some(Token::Plus) => (),
            Some(Token::Times) => {
                let result = sum * parse_part2(tokens);
                return result;
            }
            t => panic!("parsing error: {:?}", t),
        }
    }
}
