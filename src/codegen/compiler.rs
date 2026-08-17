use crate::frontend::ast::Expr;
use crate::codegen::{emitter::Emitter, regalloc::{RegisterAllocator, Reg}};

//The final value will be stored in the rax register
pub fn compile(expr: &Expr, emitter: &mut Emitter, regalloc: &mut RegisterAllocator) {
    match expr {
        Expr::Number(n) => {
            emitter.emit_mov_reg_imm64(Reg::Rax, *n);
        },
        Expr::Add(left, right) => {
            compile(left, emitter, regalloc);
            match regalloc.get_reg() {
                Ok(reg) => {
                    emitter.emit_mov_reg_reg(reg, Reg::Rax);
                    compile(right, emitter, regalloc);
                    emitter.emit_add_reg_reg(Reg::Rax, reg);
                    regalloc.free_reg(reg);
                },
                Err(_) => {
                    emitter.emit_push_reg(Reg::Rax);
                    compile(right, emitter, regalloc);
                    emitter.emit_pop_reg(Reg::Rbx);
                    emitter.emit_add_reg_reg(Reg::Rax, Reg::Rbx);
                }
            }
        },
        Expr::Sub(left, right) => {
            compile(right, emitter, regalloc);
            match regalloc.get_reg() {
                Ok(reg) => {
                    emitter.emit_mov_reg_reg(reg, Reg::Rax);
                    compile(left, emitter, regalloc);
                    emitter.emit_sub_reg_reg(Reg::Rax, reg);
                    regalloc.free_reg(reg);
                },
                Err(_) => {
                    emitter.emit_push_reg(Reg::Rax);
                    compile(left, emitter, regalloc);
                    emitter.emit_pop_reg(Reg::Rbx);
                    emitter.emit_sub_reg_reg(Reg::Rax, Reg::Rbx);
                }
            }
        },
    }
}
