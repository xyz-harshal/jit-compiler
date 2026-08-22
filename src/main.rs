//deref coercion is about function arguments and return types lining up.
//Auto-deref is about finding which type actually has the method you're calling, by peeling off layers of pointers until it finds one that fits.
mod frontend;
use frontend::parser::parse;
use frontend::ast::Expr;
mod backend;
use backend::interpreter::eval;
mod codegen;
use codegen::{emitter::Emitter, compiler::compile, regalloc::{RegisterAllocator, Reg}};
mod mem;
use mem::alloc::ExecutableMem;

fn run_jit(expr: &Expr) -> i64 {
    let mut emitter = Emitter::new();
    let mut regalloc = RegisterAllocator::new();
    compile(expr, &mut emitter, &mut regalloc);
    emitter.emit_ret();
    let bytes = emitter.finish();
    let memory = ExecutableMem::new(bytes.len()).unwrap();
    memory.write_code(bytes).unwrap();
    memory.make_executable().unwrap();
    unsafe { memory.execute_code() }
}

fn run_jit_with_print(expr: &Expr) {
    let mut emitter = Emitter::new();
    let mut regalloc = RegisterAllocator::new();
    compile(expr, &mut emitter, &mut regalloc);
    emitter.emit_mov_reg_reg(Reg::Rdi, Reg::Rax);
    let func_ptr: usize = print_val as *const () as usize;
    emitter.emit_mov_reg_imm64(Reg::Rax, func_ptr as i64);
    emitter.emit_sub_reg_imm8(Reg::Rsp, 8);
    emitter.emit_call_reg(Reg::Rax);
    emitter.emit_add_reg_imm8(Reg::Rsp, 8);
    emitter.emit_ret();
    let bytes = emitter.finish();
    let memory = ExecutableMem::new(bytes.len()).unwrap();
    memory.write_code(bytes).unwrap();
    memory.make_executable().unwrap();
    unsafe { memory.execute_code(); }
}

extern "C" fn print_val(val: i64) -> i64 {
    println!("{}", val);
    val
}

fn main() {
    let content: String = std::fs::read_to_string("input.txt").unwrap();
    match parse(&content) {
        Ok(expr) => run_jit_with_print(&expr),
        Err(err) => println!("Error: {}", err),
    }
}

#[cfg(test)]
mod jit_correctness_tests {
    use crate::frontend::parser::parse;
    use crate::backend::interpreter::eval;
    use crate::run_jit;

    fn interp_eval(src: &str) -> i64 {
        let expr = parse(src).unwrap();
        eval(&expr)
    }

    fn jit_eval(src: &str) -> i64 {
        let expr = parse(src).unwrap();
        run_jit(&expr)
    }

    fn assert_matches(src: &str) {
        let expected = interp_eval(src);
        let actual = jit_eval(src);
        assert_eq!(actual, expected, "mismatch for input {:?}", src);
    }

    #[test]
    fn number() {
        assert_matches("42");
    }

    #[test]
    fn negative_number() {
        assert_matches("-5");
    }

    #[test]
    fn simple_add() {
        assert_matches("4 + 5");
    }

    #[test]
    fn simple_sub() {
        assert_matches("10 - 3");
    }

    #[test]
    fn mixed_left_associative() {
        assert_matches("10 - 3 + 2");
    }

    #[test]
    fn deep_sub_chain() {
        assert_matches("100-1-1-1-1-1-1-1-1");
    }

    #[test]
    fn deep_add_chain_forces_spill() {
        assert_matches("1+1+1+1+1+1+1+1+1+1+1+1+1+1+1");
    }

    #[test]
    fn mixed_deep() {
        assert_matches("10-3+7-2-1+5-4");
    }

    #[test]
    fn negative_result() {
        assert_matches("1-100+50");
    }

    #[test]
    fn large_i64() {
        assert_matches("9223372036854775800+5");
    }
    #[test]
    fn simple_mul() {
        assert_matches("6 * 7");
    }
    #[test]
    fn mul_with_negative() { 
        assert_matches("-4 * 5");
    }
    #[test]
    fn mul_chain_forces_spill() {
        assert_matches("2*2*2*2*2*2*2");
    }
    #[test]
    fn mixed_add_mul() {
        assert_matches("2+3*4");
    }
    #[test]
    fn mixed_sub_mul() {
        assert_matches("10-2*3");
    }
    #[test]
    fn mixed_all_three() {
        assert_matches("10-3*2+4");
    }
    #[test]
    fn mul_by_zero() {
        assert_matches("999*0");
    }
    #[test]
    fn mul_large_values() {
        assert_matches("1000000000*3");
    }
}
