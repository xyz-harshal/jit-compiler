mod frontend;
use frontend::parser::parse;

fn main() {
    let content: String = std::fs::read_to_string("input.txt").unwrap();

    match parse(&content) {
        Ok(expr) => println!("{:?}", expr),
        Err(err) => println!("Error: {}", err),
    }
}
