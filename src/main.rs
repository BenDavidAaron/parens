
enum LispValue {
    Symbol(String),
    Number(f64),
    List(Vec<LispValue>),
}

struct Environment {
    data: HashMap<String, LispValue>,
}

fn main() {
    println!("Hello, world!");
}
