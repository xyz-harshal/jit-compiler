//deref coercion is about function arguments and return types lining up.
//Auto-deref is about finding which type actually has the method you're calling, by peeling off layers of pointers until it finds one that fits.

mod frontend;
use frontend::parser::parse;

mod codegen;
use codegen::{emitter::Emitter, compiler::compile};

mod mem;
use mem::alloc::ExecutableMem;

extern "C" fn print_val(val: i64) -> i64 {
    println!("{}", val);
    val
}

fn main() {
    let content: String = std::fs::read_to_string("input.txt").unwrap();
    match parse(&content) {
        Ok(expr) => {
            let mut emitter = Emitter::new();
            compile(&expr, &mut emitter);
            emitter.emit_mov_rdi_rax();
            let func_ptr: usize = print_val as *const () as usize;
            emitter.emit_mov_rax_imm64(func_ptr as i64);
            emitter.emit_sub_rsp_imm8(8);
            emitter.emit_call_rax();
            emitter.emit_add_rsp_imm8(8);
            emitter.emit_ret();

            let func: Vec<u8> = emitter.finish();

            let memory = ExecutableMem::new(func.len()).unwrap();
            memory.write_code(func).unwrap();
            memory.make_executable().unwrap();
            unsafe {
                memory.execute_code();
            }
        },
        Err(err) => println!("Error: {}", err),
    }
}
