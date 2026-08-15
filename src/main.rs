//deref coercion is about function arguments and return types lining up.
//Auto-deref is about finding which type actually has the method you're calling, by peeling off layers of pointers until it finds one that fits.

mod frontend;
use frontend::parser::parse;

mod codegen;
use codegen::{emitter::Emitter, compiler::compile};

mod mem;
use mem::alloc::ExecutableMem;

fn main() {
    let content: String = std::fs::read_to_string("input.txt").unwrap();
    match parse(&content) {
        Ok(expr) => {
            let mut emitter = Emitter::new();
            compile(&expr, &mut emitter);
            emitter.emit_ret();
            let func: Vec<u8> = emitter.finish();
            let memory = ExecutableMem::new(func.len()).unwrap();
            memory.write_code(func).unwrap();
            memory.make_executable().unwrap();
            println!("{:?}", unsafe { memory.execute_code() });
        },
        Err(err) => println!("Error: {}", err),
    }
}
