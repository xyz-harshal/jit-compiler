//deref coercion is about function arguments and return types lining up.
//Auto-deref is about finding which type actually has the method you're calling, by peeling off layers of pointers until it finds one that fits.

mod frontend;
use frontend::parser::parse;

mod backend;
use backend::interpreter::eval;

fn main() {
    let content: String = std::fs::read_to_string("input.txt").unwrap();
    match parse(&content) {
        Ok(expr) => {
            println!("{:?}", eval(&expr));
        },
        Err(err) => println!("Error: {}", err),
    }
}
