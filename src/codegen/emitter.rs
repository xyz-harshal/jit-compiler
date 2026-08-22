use crate::codegen::regalloc::{Reg, reg_encode};

#[derive(Debug, PartialEq, Eq)]
pub struct Emitter {
    buf: Vec<u8>,
}

impl Emitter {
    pub fn new() -> Self {
        Self {
            buf: Vec::new(),
        }
    }
    pub fn emit_ret(&mut self) {
        self.buf.push(0xC3);
    }

    pub fn emit_mov_reg_imm64(&mut self, reg: Reg, val: i64) {
        self.buf.extend_from_slice(&[0x48, 0xB8 + reg_encode(reg)]);
        self.buf.extend_from_slice(&val.to_le_bytes());
    }

    pub fn emit_mov_reg_reg(&mut self, dst: Reg, src: Reg) {
        self.buf.extend_from_slice(&[0x48, 0x89, 0xC0 + (reg_encode(src) << 3) + reg_encode(dst)]);
    }

    pub fn emit_push_reg(&mut self, reg: Reg){
        self.buf.push(0x50 + reg_encode(reg));
    }

    pub fn emit_pop_reg(&mut self, reg: Reg) {
        self.buf.push(0x58 + reg_encode(reg));
    }

    pub fn emit_add_reg_reg(&mut self, dst: Reg, src: Reg) {
        self.buf.extend_from_slice(&[0x48, 0x01, 0xC0 + (reg_encode(src) << 3) + reg_encode(dst)]);
    }

    pub fn emit_mul_reg_reg(&mut self, dst: Reg, src: Reg) {
        self.buf.extend_from_slice(&[0x48, 0x0F, 0xAF, 0xC0 + (reg_encode(dst) << 3) + reg_encode(src)]);
    }

    pub fn emit_add_reg_imm8(&mut self, reg: Reg, val: u8) {
        self.buf.extend_from_slice(&[0x48, 0x83, 0xC0 + reg_encode(reg), val]);
    }

    pub fn emit_sub_reg_reg(&mut self, dst: Reg, src: Reg) {
        self.buf.extend_from_slice(&[0x48, 0x29, 0xC0 + (reg_encode(src) << 3) + reg_encode(dst)]);
    }

    pub fn emit_sub_reg_imm8(&mut self, reg: Reg, val: u8) {
        self.buf.extend_from_slice(&[0x48, 0x83, 0xC0 + 0x28 + reg_encode(reg), val]);
    }

    pub fn emit_call_reg(&mut self, reg: Reg) {
        self.buf.extend_from_slice(&[0xFF, 0xC0 + 0x10 + reg_encode(reg)]);
    }

    pub fn finish(self) -> Vec<u8> {
        self.buf
    }
}

#[cfg(test)]
mod tests {
    use crate::codegen::{emitter::Emitter, regalloc::{Reg, reg_encode}};
    #[test]
    fn mov_number() {
        let mut emitter = Emitter::new();
        emitter.emit_mov_reg_imm64(Reg::Rax, 69);
        emitter.emit_ret();
        assert_eq!(emitter.finish(), [0x48, 0xB8, 69, 0, 0, 0, 0, 0, 0, 0, 0xC3]);
    }
}
