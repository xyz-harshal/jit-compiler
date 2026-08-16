use crate::frontend::ast::Expr;
use crate::codegen::emitter::Emitter;

//The final value will be stored in the rax register
pub fn compile(expr: &Expr, emitter: &mut Emitter) {
    match expr {
        Expr::Number(n) => {
            emitter.emit_mov_rax_imm64(*n);
        },
        Expr::Add(left, right) => {
            compile(left, emitter);
            emitter.emit_push_rax();
            compile(right, emitter);
            emitter.emit_pop_rbx();
            emitter.emit_add_rax_rbx();
        },
        Expr::Sub(left, right) => {
            compile(right, emitter);
            emitter.emit_push_rax();
            compile(left, emitter);
            emitter.emit_pop_rbx();
            emitter.emit_sub_rax_rbx();
        },
    }
}
