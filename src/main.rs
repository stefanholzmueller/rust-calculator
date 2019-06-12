use combine::{many1, choice};
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
        Ok(result) => println!("result = {:?}", result),
        Err(err) => println!("error: {}", err)
    }
}

type Num = Ratio<BigInt>;

enum Expr {
    Nat(Num),
    Plus(Box<Expr>, Box<Expr>),
}

fn parse(input: String) -> Result<Expr, String> {
    println!("input = {}", input);
    let slice: &str = &input[..];
    let number_parser = many1(digit()).map(parse_number);
    let plus_parser = (
        many1(digit()).map(parse_number), // TODO remove duplication
        char('+'),
        many1(digit()).map(parse_number)
      ).map(|(n, _p, o)| Expr::Plus(Box::new(n), Box::new(o)));
    let mut parser = choice((plus_parser, number_parser));
    //let mut parser = or(plus_parser, number_parser); // TODO number literals are not parsed anymore
    let result = parser.easy_parse(slice).map(|r| r.0);
    result.map_err(|e| e.to_string())
}

fn parse_number(input: Vec<char>) -> Expr {
    let str: String = input.into_iter().collect();
    let bi: BigInt = str.parse().unwrap();
    Expr::Nat(Ratio::new(bi, BigInt::from(1)))
}

fn eval(expr: Expr) -> Ratio<BigInt> {
    match expr {
        Expr::Nat(ratio) => ratio,
        Expr::Plus(a, b) => eval(*a) + eval(*b)
    }
}
