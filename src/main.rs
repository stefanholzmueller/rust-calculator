use combine::many1;
use combine::Parser;
use combine::parser::byte::digit;

use num_bigint::BigInt;
use num_rational::Ratio;


fn main() {
    let args = std::env::args().skip(1);
    let input = args.collect::<Vec<String>>().join(" ");
    let expr = parse(input);
    let result = eval(expr);
    println!("= {}", result);
}

type Num = Ratio<BigInt>;

enum Expr {
    Nat(Num),
}

fn parse(input: String) -> Expr {
    let slice: &str = &input[..];
    let parser = many1(digit());
    let result = parser.easy_parse(slice);
    match result {
        Ok((value, _remaining_input)) => println!("{:?}", value),
        Err(err) => println!("error: {}", err)
    }
    Expr::Nat(Ratio::new(BigInt::from(123), BigInt::from(1)))
}

fn eval(expr: Expr) -> Ratio<BigInt> {
    match expr {
        Expr::Nat(ratio) => ratio,
    }
}

