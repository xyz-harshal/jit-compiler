//deref coercion is about function arguments and return types lining up.
//Auto-deref is about finding which type actually has the method you're calling, by peeling off layers of pointers until it finds one that fits.

mod frontend;
use frontend::parser::parse;

mod backend;
use backend::interpreter::eval;

mod codegen;
use codegen::emitter::Emitter;

fn main() {
    let content: String = std::fs::read_to_string("input.txt").unwrap();
    match parse(&content) {
        Ok(expr) => {
            println!("{:?}", eval(&expr));
            let mut emitter = Emitter::new();
            emitter.emit_mov_rax_imm64(eval(&expr));
            emitter.emit_ret();
            println!("{:?}", emitter.finish());
        },
        Err(err) => println!("Error: {}", err),
    }
}
