#[derive(Clone, Copy)]
struct Core {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    ip: u32,
    sp: u32,
    lr: u32,
    pc: u32,
}

#[derive(Clone, Copy)]
pub struct Cursor {
    registers: Core,
}

impl Cursor {
    pub fn up(&self) -> Cursor {
        *self
    }

    pub fn pc(&self) -> u64 {
        self.registers.pc as u64
    }
}
