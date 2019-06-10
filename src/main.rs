use combine::many1;
use combine::parser::char::digit;
use combine::parser::Parser;
use num_bigint::BigInt;
use num_rational::Ratio;
use combine::parser::repeat::Many1;

fn main() {
    let args = std::env::args().skip(1);
    let input = args.collect::<Vec<String>>().join(" ");
    let expr_result = parse(input);
    let calc_result = expr_result.map(eval);
    match calc_result {
        Ok(result) => println!("{:?}", result),
        Err(err) => println!("error: {}", err)
    }
}

type Num = Ratio<BigInt>;

enum Expr {
    Nat(Num),
}

fn parse(input: String) -> Result<Expr, String> {
    println!("input = {}", input);
    let slice: &str = &input[..];
    let mut parser: Many1<Vec<_>, _> = many1(digit());
    let result = parser.easy_parse(slice).map(|r| r.0);
    let expr = result.map(parse_number);
    return expr.map_err(|e| e.to_string());
}

fn parse_number(input: Vec<char>) -> Expr {
    let str: String = input.into_iter().collect();
    let bi: BigInt = str.parse().expect("CRAP");
    return Expr::Nat(Ratio::new(bi, BigInt::from(1)));
}

fn eval(expr: Expr) -> Ratio<BigInt> {
    match expr {
        Expr::Nat(ratio) => ratio,
    }
}
