use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
enum LispValue {
    Symbol(String),
    Number(f64),
    List(Vec<LispValue>),
}

impl FromStr for LispValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().peekable();
        let result = parse_expression(&mut chars)?;
        if chars.next().is_none() {
            Ok(result)
        } else {
            Err("Unexpected characters at the end of input".to_string())
        }
    }
}

fn parse_expression<I>(chars: &mut std::iter::Peekable<I>) -> Result<LispValue, String>
where
    I: Iterator<Item = char>,
{
    //Parsing Logic Goes Here
    unimplemented!("TODO: Parsing Logic");
}

struct Environment {
    data: HashMap<String, LispValue>,
}

fn main() {
    let input = "(+ 2 3)";
    match input.parse::<LispValue>() {
        Ok(expr) => println!("Parsed: {:?}", expr),
        Err(err) => println!("Parse error: {}", err),
    }
}
