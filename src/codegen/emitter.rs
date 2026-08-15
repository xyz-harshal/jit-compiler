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
    pub fn emit_mov_rax_imm64(&mut self, val: i64) {
        self.buf.push(0x48); //REX.W prefix for using the x86_32 opcode in x86_64 architecture.
        self.buf.push(0xB8); //The opcode for MOV
        self.buf.extend_from_slice(&val.to_le_bytes()); //pushing the values in an little endian way.
    }
    pub fn emit_push_rax(&mut self) {
        self.buf.push(0x50); //The opcode for PUSH from rax
    }
    pub fn emit_pop_rbx(&mut self) {
        self.buf.push(0x5B); //The opcode to POP and store it in rbx
    }
    pub fn emit_add_rax_rbx(&mut self) {
        self.buf.push(0x48);
        self.buf.push(0x01); //The opcode to ADD any 2 registers or memloc
        self.buf.push(0xD8); //MOD-RM to state whether registers or memloc and the byte encoding of the src and des.
    }
    pub fn emit_sub_rax_rbx(&mut self) {
        self.buf.push(0x48);
        self.buf.push(0x29); //The opcode to SUB any 2 regsiters or memloc
        self.buf.push(0xD8);
    }
    pub fn finish(self) -> Vec<u8> {
        self.buf
    }
}

#[cfg(test)]
mod tests {
    use crate::codegen::emitter::Emitter;
    #[test]
    fn mov_number() {
        let mut emitter = Emitter::new();
        emitter.emit_mov_rax_imm64(69);
        emitter.emit_ret();
        assert_eq!(emitter.finish(), [0x48, 0xB8, 69, 0, 0, 0, 0, 0, 0, 0, 0xC3]);
    }
}
