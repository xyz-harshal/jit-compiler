#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Reg {
    Rax,
    Rcx,
    Rdx,
    Rbx,
    Rsp,
    Rbp,
    Rsi,
    Rdi,
}

pub fn reg_encode(reg: Reg) -> u8 {
    match reg {
        Reg::Rax => 0,
        Reg::Rcx => 1,
        Reg::Rdx => 2,
        Reg::Rbx => 3,
        Reg::Rsp => 4,
        Reg::Rbp => 5,
        Reg::Rsi => 6,
        Reg::Rdi => 7,
    }
}

pub struct RegisterAllocator {
    pool: Vec<Reg>,
}

impl RegisterAllocator {
    pub fn new() -> Self {
        Self {
            pool: vec![Reg::Rcx, Reg::Rdx, Reg::Rbx, Reg::Rsi],
        }
    }

    pub fn get_reg(&mut self) -> Result<Reg,String> {
        if self.pool.is_empty() {
            return Err("No registers Available".to_string());
        }
        Ok(self.pool.pop().unwrap())
    }
    pub fn free_reg(&mut self, reg: Reg) {
        self.pool.push(reg);
    }
}
