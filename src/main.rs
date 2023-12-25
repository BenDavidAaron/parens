use nom::{alt, char, delimited, digit, map_res, named, recognize, seperated_list, take_while1};
use std::collections::HashMap;

enum LispValue {
    Symbol(String),
    Number(f64),
    List(Vec<LispValue>),
}

struct Environment {
    data: HashMap<String, LispValue>,
}

named!(
    parse_lisp_expr<LispValue>,
    alt!(
        parse_number => { |n| LispValue::Number(n) } |
        parse_symbol => { |s| LispValue::Symbol(s) } |
        parse_list
    )
);

named!(
    parse_number<f64>,
    map_res!(
        recognize!(pair!(opt!(alt!(char!('+') | char!('-'))), digit)),
        |s: &str| s.parse::<f64>()
    )
);

named!(
    parse_symbol<String>,
    map!(
        take_while1!(is_valid_symbol_char),
        |s : &[u8]| String::from_utf8_lossy(s).into_owned()
    )
);

named!(
    parse_list<LispValue>,
    delimited(
        char!('('),
        seperated_list!(char!(' '), parse_lisp_expr),
        char!(')')
    )
);

fn main() {
    println!("Hello, world!");
}
