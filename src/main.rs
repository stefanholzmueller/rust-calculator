use combine::{many1, choice, attempt};
use combine::parser::char::{char, digit};
use combine::parser::Parser;
use num_bigint::BigInt;
use num_rational::Ratio;

fn main() {
    let args = std::env::args().skip(1);
    let input = args.collect::<Vec<String>>().join(" ");
    let expr_result = parse(input);
    let calc_result = expr_result.map(eval);
    match calc_result {
        Ok(result) => println!("result = {}", result.to_string()),
        Err(err) => println!("error: {}", err)
    }
}

type Num = Ratio<BigInt>;

enum Expr {
    Nat(Num),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
}

fn parse(input: String) -> Result<Expr, String> {
    let new_number_parser = || many1(digit()).map(parse_number);
    let plus_parser = (new_number_parser(), char('+'), new_number_parser())
        .map(|(a, _p, b)| Expr::Plus(Box::new(a), Box::new(b)));
    let minus_parser = (new_number_parser(), char('-'), new_number_parser())
        .map(|(a, _p, b)| Expr::Minus(Box::new(a), Box::new(b)));
    let mut parser = choice((
        attempt(plus_parser),
        attempt(minus_parser),
        new_number_parser(),
    ));

    let slice: &str = &input[..];
    let result = parser.easy_parse(slice).map(|r| r.0);
    result.map_err(|e| e.to_string())
}
/*
fn new_number_parser() -> Map<Many1<_,_>,_> {
    return many1(digit()).map(parse_number);
}

fn new_binary_parser(op: char, f: F) where F: Fn(Num, Num) -> Expr {
    return (new_number_parser(), char(op), new_number_parser()).map(f);
}
*/
fn parse_number(input: Vec<char>) -> Expr {
    let str: String = input.into_iter().collect();
    let bi: BigInt = str.parse().unwrap();
    Expr::Nat(Ratio::from_integer(bi))
}

fn eval(expr: Expr) -> Ratio<BigInt> {
    match expr {
        Expr::Nat(ratio) => ratio,
        Expr::Plus(a, b) => eval(*a) + eval(*b),
        Expr::Minus(a, b) => eval(*a) - eval(*b),
    }
}
